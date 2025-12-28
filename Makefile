.PHONY: run build doc doc-serve 
.DEFAULT: help


include scripts/colorprint


help: ## Show this help message
	@echo "Available commands:"
# 	echo $(MAKEFILE_LIST)
#	was $(MAKEFILE_LIST) but fails if there is an include. So we use Makefile
	@grep -E '^[ a-zA-Z_0-9-]+:.*?## .*$$' Makefile | awk 'BEGIN {FS = ":.*?## "}; {printf "$(Red)%-20s$(Color_Off) : $(Blue)%s$(Color_Off)\n", $$1, $$2}'


test: build
	echo "" > yamake.log
	cargo fmt && cargo test ${args}


build: ## build 
	@printf "\n$(White)$(On_Blue)build$(Color_Off)\n"
	(  cargo fmt && 	cargo build  )
	@printf "finished building\n"

install: build ## install 
	@printf "\n$(White)$(On_Blue)install$(Color_Off)\n"
	( cargo fmt && cargo install --path . )
	@printf "finished installing\n"	

clean:
	rm -rf $(sandbox)
	mkdir $(sandbox)


doc-serve: ## run mdbook and serve
	rm -f doc/mermaid-animate.js
	( cd doc && mdbook-mermaid-animate install && mdbook serve )

