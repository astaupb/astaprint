debug:
	cargo build --all
release:
	cargo build --all --release

deploy: deploy_release

deploy_debug: debug
	./flushall.sh
	#sunrise
	ssh astaprint@sunrise "systemctl --user stop backend && rm /home/astaprint/bin/backend"
	scp ./target/debug/backend astaprint@sunrise:/home/astaprint/bin
	ssh astaprint@sunrise "systemctl --user daemon-reload && systemctl --user start backend"
	#shiva
	ssh astaprint@shiva "systemctl --user stop backend && rm /home/astaprint/bin/backend"
	scp ./target/debug/backend astaprint@shiva:/home/astaprint/bin
	ssh astaprint@shiva "systemctl --user daemon-reload && systemctl --user start backend"
	#amnesia
	ssh astaprint@amnesia "systemctl --user stop dispatcher && rm /home/astaprint/bin/dispatcher"
	scp ./target/debug/dispatcher astaprint@amnesia:/home/astaprint/bin
	ssh astaprint@amnesia "systemctl --user daemon-reload && systemctl --user start dispatcher"
	#widow
	ssh astaprint@widow "systemctl --user stop worker && rm /home/astaprint/bin/worker"
	scp ./target/debug/worker astaprint@widow:/home/astaprint/bin
	ssh astaprint@widow "systemctl --user daemon-reload && systemctl --user start worker"
	./flushall.sh

deploy_release: release
	./flushall.sh
	#shiva
	ssh astaprint@shiva "systemctl --user stop backend && rm /home/astaprint/bin/backend"
	scp ./target/release/backend astaprint@shiva:/home/astaprint/bin
	ssh astaprint@shiva "systemctl --user daemon-reload && systemctl --user start backend"
	#amnesia
	ssh astaprint@amnesia "systemctl --user stop dispatcher && rm /home/astaprint/bin/dispatcher"
	scp ./target/release/dispatcher astaprint@amnesia:/home/astaprint/bin
	ssh astaprint@amnesia "systemctl --user daemon-reload && systemctl --user start dispatcher"
	#widow
	ssh astaprint@widow "systemctl --user stop worker && rm /home/astaprint/bin/worker"
	scp ./target/release/worker astaprint@widow:/home/astaprint/bin
	ssh astaprint@widow "systemctl --user daemon-reload && systemctl --user start worker"
	./flushall.sh

test:
	#sunrise
	ssh astaprint@sunrise "systemctl --user stop backend && rm /home/astaprint/bin/backend"
	scp ./target/debug/backend astaprint@sunrise:/home/astaprint/bin
	ssh astaprint@sunrise "systemctl --user daemon-reload && systemctl --user start backend"


singles:
	cd mysql && cargo build
	cd sodium && cargo build
	cd logger && cargo build
	cd model && cargo build
	cd pdf && cargo build
	cd legacy && cargo build
	cd redis && cargo build
	cd snmp && cargo build

.PHONY: singles binaries deploy

clean:
	rm -rf target



