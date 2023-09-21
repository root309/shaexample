extern crate sha2;
extern crate hex;

use sha2::{Sha256, Digest};

// ハッシュ関数の決定性に基づいて同じ入力データに対しては常に同じハッシュ値を生成する
fn main() {
    let mut hasher = Sha256::new();

    // 文字列"Hello World"に対するハッシュを計算
    hasher.update(b"Hello World");
    let result = hasher.finalize();
    let hash_str1 = hex::encode(result);
    println!("SHA-256 hash of 'Hello World': {}", hash_str1);

    // hashをリセット
    hasher = Sha256::new();

    // 文字列"Hello World,"に対するハッシュを計算
    hasher.update(b"Hello World,");
    let result = hasher.finalize();
    let hash_str2 = hex::encode(result);
    println!("SHA-256 hash of 'Hello World,': {}", hash_str2);
}
