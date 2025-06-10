
# Algum comentário

clean:
	@echo "Limpando compilação do projeto"
	@rm -rf pkg
	@rm -rf target
	@rm Cargo.lock

build:
	@echo "Compilando projeto para o target wasm"
	@wasm-pack build --target web 