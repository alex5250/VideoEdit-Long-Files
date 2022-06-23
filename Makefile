install_console_linux:
		./installer/install.sh
build_comment_request:
		mkdir bin &
		./comment_request/build.sh
clean:
	rm -rf node_modules
	rm -rf ./comment_request/target
	rm -rf ./CutterDesktop/CutterDesktopV1/node_modules
	rm -rf ./CutterDesktop/CutterDesktopV1/src-tauri/target
	rm -rf bin

build_desktop:
	./CutterDesktop/CutterDesktopV1/build.sh

	