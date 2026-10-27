.PHONY: build test install clean
build:
	python3 -m compileall -q src
test:
	python3 -m unittest discover -s tests -v
install:
	install -Dm755 bin/sigma-mac-run $(DESTDIR)/usr/bin/sigma-mac-run
clean:
	rm -rf src/__pycache__ tests/__pycache__
