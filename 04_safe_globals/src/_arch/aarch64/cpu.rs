use aarch64_cpu::asm;


// 返回类型为 !，表示该函数永不返回（diverges），典型的场景是无限循环或导致程序终止的操作。

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe(); // 假定 asm 是某个模块的别名，具体功能可能映射到低级 CPU 指令。
    }
}

