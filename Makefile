build:
	docker build . -t xyo-sdk-rust:latest


ssh:
	docker run --name xyo_sdk_rust -it --rm -v $(PWD):/var/local/xyo-sdk xyo-sdk-rust:latest sh
