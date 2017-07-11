#![feature(test)]
extern crate test;

pub mod v1;
pub mod v2;
pub mod v3;
pub mod v4;
pub mod v5;
pub mod v6;
pub mod v7;
pub mod v8;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn iters() -> u64 {
        (1..10000001).filter(|x| x % 2 == 0).sum()
    }

    fn loops() -> u64 {
        let mut i = 1;
        let mut total = 0;
        while i < 10000001 {
            if i % 2 == 0 {
                total += i;
            }
            i += 1;
        }
        total
    }

    #[test]
    fn it_works() {
        assert_eq!(iters(), loops());
        assert_eq!(iters(), v1::do_it());
        assert_eq!(iters(), v2::do_it());
        assert_eq!(iters(), v3::do_it());
        assert_eq!(iters(), v4::do_it());
        assert_eq!(iters(), v5::do_it());
        assert_eq!(iters(), v6::do_it());
        assert_eq!(iters(), v7::do_it());
        assert_eq!(iters(), v8::do_it());
    }

    #[bench]
    fn bench_v1(b: &mut Bencher) {
        b.iter(v1::do_it)
    }

    #[bench]
    fn bench_v2(b: &mut Bencher) {
        b.iter(v2::do_it)
    }

    #[bench]
    fn bench_v3(b: &mut Bencher) {
        b.iter(v3::do_it)
    }

    #[bench]
    fn bench_v4(b: &mut Bencher) {
        b.iter(v4::do_it)
    }

    #[bench]
    fn bench_v5(b: &mut Bencher) {
        b.iter(v5::do_it)
    }

    #[bench]
    fn bench_v6(b: &mut Bencher) {
        b.iter(v6::do_it)
    }

    #[bench]
    fn bench_v7(b: &mut Bencher) {
        b.iter(v7::do_it)
    }

    #[bench]
    fn bench_v8(b: &mut Bencher) {
        b.iter(v8::do_it)
    }

    #[bench]
    fn bench_zzz_iters(b: &mut Bencher) {
        b.iter(iters)
    }

    #[bench]
    fn bench_zzz_loops(b: &mut Bencher) {
        b.iter(loops)
    }
}
