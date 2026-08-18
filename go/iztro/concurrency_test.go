package iztro

import (
	"context"
	"errors"
	"strings"
	"sync"
	"testing"
)

// 并发与性能测试。
//
// wasm 实例池让并发调用各占一个实例，实例内存互不相干；这里用混合调用压一遍，
// 配合 -race 检查跨 goroutine 的共享状态（引擎初始化、池收发、编译产物）。

// TestConcurrentMixedCalls 用 64 个 goroutine 混合调用各入口，校验结果正确且无数据竞争。
func TestConcurrentMixedCalls(t *testing.T) {
	const goroutines = 64
	const rounds = 4

	var wg sync.WaitGroup
	errCh := make(chan error, goroutines*rounds)
	for g := 0; g < goroutines; g++ {
		wg.Add(1)
		go func(g int) {
			defer wg.Done()
			for r := 0; r < rounds; r++ {
				if err := mixedCallRound(g); err != nil {
					errCh <- err
					return
				}
			}
		}(g)
	}
	wg.Wait()
	close(errCh)
	for err := range errCh {
		t.Error(err)
	}
}

// mixedCallRound 按 goroutine 序号轮转，覆盖排盘、运限、prompt、查询与错误路径。
func mixedCallRound(g int) error {
	switch g % 5 {
	case 0:
		chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
		if err != nil {
			return err
		}
		if chart.Palace(PalaceSoul).NameKey != PalaceSoul || !chart.Palace(PalaceSoul).Has(StarZiweiMaj) {
			return errors.New("并发排盘结果不正确")
		}
	case 1:
		chart, err := ByLunar("2000-7-17", 2, GenderFemale, NotLeapMonth, LanguageEnUS, nil)
		if err != nil {
			return err
		}
		if chart.SolarDate != "2000-8-16" {
			return errors.New("并发农历排盘结果不正确: " + chart.SolarDate)
		}
	case 2:
		chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
		if err != nil {
			return err
		}
		h, err := chart.Horoscope("2024-10-1", 0)
		if err != nil {
			return err
		}
		if h.Yearly.HeavenlyStemKey != StemJia || h.Yearly.EarthlyBranchKey != BranchChen {
			return errors.New("并发运限结果不正确")
		}
	case 3:
		chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
		if err != nil {
			return err
		}
		prompt, err := chart.AstrolabeToPrompt()
		if err != nil {
			return err
		}
		if !strings.Contains(prompt, "五行局: 木三局") {
			return errors.New("并发 prompt 结果不正确")
		}
	default:
		// 错误路径同样并发：非法入参不得污染其他实例
		if _, err := BySolar("2000-13-1", 2, GenderMale, true, LanguageZhCN, nil); !errors.Is(err, ErrInvalidDate) {
			return errors.New("并发非法日期未返回 ErrInvalidDate")
		}
		zodiac, err := GetZodiacBySolarDate("2000-8-16", LanguageZhCN, nil)
		if err != nil {
			return err
		}
		if zodiac != "龙" {
			return errors.New("并发生肖查询结果不正确: " + zodiac)
		}
	}
	return nil
}

// TestWarmupAndCacheDir 校验预热可重复调用，并报告编译缓存目录。
func TestWarmupAndCacheDir(t *testing.T) {
	ctx := context.Background()
	for i := 0; i < 2; i++ {
		if err := Warmup(ctx); err != nil {
			t.Fatalf("Warmup: %v", err)
		}
	}
	dir, err := CompilationCacheDir(ctx)
	if err != nil {
		t.Fatalf("CompilationCacheDir: %v", err)
	}
	t.Logf("编译缓存目录: %q", dir)

	// 预热后照常排盘
	if _, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil); err != nil {
		t.Fatalf("预热后排盘: %v", err)
	}
}

// TestContextCancellation 校验已取消的 Context 会在进入 wasm 前失败。
func TestContextCancellation(t *testing.T) {
	if err := Warmup(context.Background()); err != nil {
		t.Fatalf("Warmup: %v", err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	cancel()

	if _, err := BySolarContext(ctx, "2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil); !errors.Is(err, ErrInternal) {
		t.Errorf("已取消的 Context 应返回 ErrInternal，实际 %v", err)
	}
	chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := chart.HoroscopeContext(ctx, "2024-10-1", 0); err == nil {
		t.Error("已取消的 Context 下运限应报错")
	}
	if _, err := chart.AstrolabeToPromptContext(ctx); err == nil {
		t.Error("已取消的 Context 下 prompt 应报错")
	}
	if _, err := chart.RearrangedContext(ctx, StemGeng, BranchChen); err == nil {
		t.Error("已取消的 Context 下重排应报错")
	}
}

// BenchmarkBySolarSerial 测串行排盘吞吐。
func BenchmarkBySolarSerial(b *testing.B) {
	if err := Warmup(context.Background()); err != nil {
		b.Fatal(err)
	}
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		if _, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil); err != nil {
			b.Fatal(err)
		}
	}
}

// BenchmarkBySolarParallel 测并发排盘吞吐；与串行版对比即实例池的收益。
func BenchmarkBySolarParallel(b *testing.B) {
	if err := Warmup(context.Background()); err != nil {
		b.Fatal(err)
	}
	b.ResetTimer()
	b.RunParallel(func(pb *testing.PB) {
		for pb.Next() {
			if _, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil); err != nil {
				b.Fatal(err)
			}
		}
	})
}

// BenchmarkPalaceHas 测宫位星耀包含判断；星耀集合在 link 时预计算，此处只查表。
func BenchmarkPalaceHas(b *testing.B) {
	chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		b.Fatal(err)
	}
	soul := chart.Palace(PalaceSoul)
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		if !soul.Has(StarZiweiMaj) {
			b.Fatal("命宫应有紫微")
		}
	}
}

// BenchmarkFixIndex 测纯 Go 实现的索引回绕；对照它原本要往返一次 wasm。
func BenchmarkFixIndex(b *testing.B) {
	for i := 0; i < b.N; i++ {
		if _, err := FixIndex(i, 12); err != nil {
			b.Fatal(err)
		}
	}
}

// TestCloseDuringConcurrentCalls 在持续排盘的过程中反复 Close：
// Close 先摘引擎再等在飞调用归还，因此在飞调用要么正常完成、
// 要么在引擎重新初始化后完成，不会撞上被关掉的实例（trap 或 panic）。
func TestCloseDuringConcurrentCalls(t *testing.T) {
	const goroutines = 16

	stop := make(chan struct{})
	errCh := make(chan error, goroutines)
	var wg sync.WaitGroup
	for i := 0; i < goroutines; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for {
				select {
				case <-stop:
					return
				default:
				}
				chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
				if err != nil {
					errCh <- err
					return
				}
				if len(chart.Palaces) != 12 {
					errCh <- errors.New("排盘结果宫位数不为 12")
					return
				}
			}
		}()
	}

	for i := 0; i < 5; i++ {
		if err := Close(context.Background()); err != nil {
			t.Errorf("第 %d 次 Close: %v", i, err)
		}
	}

	close(stop)
	wg.Wait()
	close(errCh)
	for err := range errCh {
		t.Error(err)
	}

	// 关闭后照常排盘
	if _, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil); err != nil {
		t.Fatalf("Close 后排盘: %v", err)
	}
}

// TestCloseRespectsCanceledContext 校验 ctx 已取消时不关闭、引擎照常可用。
func TestCloseRespectsCanceledContext(t *testing.T) {
	if err := Warmup(context.Background()); err != nil {
		t.Fatal(err)
	}
	ctx, cancel := context.WithCancel(context.Background())
	cancel()

	if err := Close(ctx); !errors.Is(err, ErrInternal) {
		t.Errorf("已取消的 Context 下 Close 应返回 ErrInternal，实际 %v", err)
	}
	if _, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil); err != nil {
		t.Fatalf("Close 未执行时引擎应照常可用: %v", err)
	}
}
