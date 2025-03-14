fn main() {
    tauri_build::build();
    
    // 当 schema.prisma 文件发生变化时重新运行构建脚本
    println!("cargo:rerun-if-changed=../prisma/schema.prisma");
}
