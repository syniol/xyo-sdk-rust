build:
	docker build . -t xyo-sdk-rust:latest


ssh:
	docker run -it --rm -v $(PWD):/var/local/xyo-sdk xyo-sdk-rust:latest sh
