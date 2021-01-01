extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use vujio_client::*;

    #[test]
    fn it_works() {
        client::bundle(
            "src/main.ts",
            &client::BundlerConfig {
                minify: true,
                compat: true,
                source_maps: true,
            },
        );
    }

    #[bench]
    fn bench_compiler(b: &mut Bencher) {
        b.iter(|| {
            client::bundle(
                "src/main.ts",
                &client::BundlerConfig {
                    minify: true,
                    compat: true,
                    source_maps: true,
                },
            )
        });
    }
}
