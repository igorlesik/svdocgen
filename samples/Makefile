.PHONY: all sample1 sample2
all: sample1 sample2

sample1:
	@mkdir -p ${TARGET_DIR}/$@
	${SVDOCGEN} -o ${TARGET_DIR}/$@ -i sample1 And3.sv

sample2:
	@mkdir -p ${TARGET_DIR}/$@
	${SVDOCGEN} -o ${TARGET_DIR}/$@ -i sample1 And3.sv Mux2.sv i/do/not/exist And3.sv