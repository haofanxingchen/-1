题目：对于Starry的内核复现和测试补充
组员：吴昊
目标描述：为Starry已有的部分进行内核复现与测试补充（优先考虑网络链接与优化）。
以完成部分：
完成代码拉取（axpoll），本地跑通测试。
修复了一个死锁的bug,并且补充了对应的测试。新增了关于
    will_wake 去重路径：同一个 waker 注册 65 次，wake() 一次 → 返回 64、计数 == 64（不是 65）。验证"满时同 waker 不重复唤醒"。
    大循环环绕：注册 200 个不同 waker 再 wake() 一次 → 返回 64、200 个计数器总和 == 200、无 panic/越界。
    Wake for PollSet：把 Arc<PollSet> 直接当 Waker 用，验证它能转发唤醒。
    IoEvents 位标志：ALWAYS_POLL == ERR | HUP、IN|OUT 组合、contains。
    部分填充计数：注册 3 个 → wake() 返回 3、计数 == 3。
    并发压力：多线程 register + 另一线程 wake，断言无死锁/panic
的测试（原代码通过）
开始性能基准测试和性能改进,完成测试。
基础结果：
register (full set)      10.0 ns/op
1 reg + wake()           51.0 ns/op
64 reg + wake()         688.2 ns/op
进行优化：
把 wake() 从"每次排空都分配/释放 1KB"改成"复用缓冲区"：
新增 Inner::take_all()：把已注册的 waker 移到栈上临时数组，重置 cursor，保留 Box 缓冲不释放。
wake() 不再 Inner::new()，改为 take_all 后出锁再逐个唤醒（保持了之前修的"锁外唤醒"纪律）。
register (full set)       9.1 ns/op
1 reg + wake()           16.9 ns/op
64 reg + wake()         629.6 ns/op


之后计划，做进一步尝试提高性能