include make.mk

debug:
	cargo build --all
release:
	cargo build --all --release

deploy: deploy_release

deploy_debug: debug
	#backend
	ssh $(backend_user)@$(backend_host) "systemctl --user stop backend && rm $(backend_dir)/bin/backend"
	scp ./target/debug/backend $(backend_user)@$(backend_host):$(backend_dir)/bin
	ssh $(backend_user)@$(backend_host) "systemctl --user daemon-reload && systemctl --user start backend"
	#dispatcher
	ssh $(dispatcher_user)@$(dispatcher_host) "systemctl --user stop dispatcher && rm $(dispatcher_dir)/bin/dispatcher"
	scp ./target/debug/dispatcher $(dispatcher_user)@$(dispatcher_host):$(dispatcher_dir)/bin
	ssh $(dispatcher_user)@$(dispatcher_host) "systemctl --user daemon-reload && systemctl --user start dispatcher"
	#worker
	ssh $(worker_user)@$(worker_host) "systemctl --user stop worker && rm $(worker_dir)/bin/worker"
	scp ./target/debug/worker $(worker_user)@$(worker_host):$(worker_dir)/bin
	ssh $(worker_user)@$(worker_host) "systemctl --user daemon-reload && systemctl --user start worker"

deploy_release: release
	#backend
	ssh $(backend_user)@$(backend_host) "systemctl --user stop backend && rm $(backend_dir)/bin/backend"
	scp ./target/release/backend $(backend_user)@$(backend_host):$(backend_dir)/bin
	ssh $(backend_user)@$(backend_host) "systemctl --user daemon-reload && systemctl --user start backend"
	#dispatcher
	ssh $(dispatcher_user)@$(dispatcher_host) "systemctl --user stop dispatcher && rm $(dispatcher_dir)/bin/dispatcher"
	scp ./target/release/dispatcher $(dispatcher_user)@$(dispatcher_host):$(dispatcher_dir)/bin
	ssh $(dispatcher_user)@$(dispatcher_host) "systemctl --user daemon-reload && systemctl --user start dispatcher"
	#worker
	ssh $(worker_user)@$(worker_host) "systemctl --user stop worker && rm $(worker_dir)/bin/worker"
	scp ./target/release/worker $(worker_user)@$(worker_host):$(worker_dir)/bin
	ssh $(worker_user)@$(worker_host) "systemctl --user daemon-reload && systemctl --user start worker"

support:
	#dispatcher
	ssh $(debug_dispatcher_user)@$(debug_dispatcher_host) "systemctl --user stop dispatcher && rm $(debug_dispatcher_dir)/bin/dispatcher && cp .prodenv .env"
	scp ./target/release/dispatcher $(debug_dispatcher_user)@$(debug_dispatcher_host):$(debug_dispatcher_dir)/bin
	ssh $(debug_dispatcher_user)@$(debug_dispatcher_host) "systemctl --user daemon-reload && systemctl --user start dispatcher"

test: debug
	#backend
	ssh $(debug_backend_user)@$(debug_backend_host) "systemctl --user stop backend && rm $(debug_backend_dir)/bin/backend"
	scp ./target/debug/backend $(debug_backend_user)@$(debug_backend_host):$(debug_backend_dir)/bin
	ssh $(debug_backend_user)@$(debug_backend_host) "systemctl --user daemon-reload && systemctl --user start backend"
	#dispatcher
	ssh $(debug_dispatcher_user)@$(debug_dispatcher_host) "systemctl --user stop dispatcher && rm $(debug_dispatcher_dir)/bin/dispatcher && cp .testenv .env"
	scp ./target/debug/dispatcher $(debug_dispatcher_user)@$(debug_dispatcher_host):$(debug_dispatcher_dir)/bin
	ssh $(debug_dispatcher_user)@$(debug_dispatcher_host) "systemctl --user daemon-reload && systemctl --user start dispatcher"
	#worker
	ssh $(debug_worker_user)@$(debug_worker_host) "systemctl --user stop worker && rm $(debug_worker_dir)/bin/worker"
	scp ./target/debug/worker $(debug_worker_user)@$(debug_worker_host):$(debug_worker_dir)/bin
	ssh $(debug_worker_user)@$(debug_worker_host) "systemctl --user daemon-reload && systemctl --user start worker"

restart: flushall
	#backend
	ssh $(backend_user)@$(backend_host) "systemctl --user daemon-reload && systemctl --user restart backend"
	#dispatcher
	ssh $(dispatcher_user)@$(dispatcher_host) "systemctl --user daemon-reload && systemctl --user restart dispatcher"
	#worker
	ssh $(worker_user)@$(worker_host) "systemctl --user daemon-reload && systemctl --user restart worker"

flushall:
	redis-cli -h $(worker_host) -a $(worker_redis_auth) flushall
	redis-cli -h $(dispatcher_host) -a $(dispatcher_redis_auth) flushall
	redis-cli -h $(backend_host) -a $(backend_redis_auth) flushall
	redis-cli -h $(database_host) -a $(database_redis_auth) flushall
singles:
	cd mysql && cargo build
	cd sodium && cargo build
	cd logger && cargo build
	cd model && cargo build
	cd pdf && cargo build
	cd redis && cargo build
	cd snmp && cargo build

.PHONY: singles binaries deploy

clean:
	rm -rf target



