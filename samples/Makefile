.PHONY: all sample1 sample2 sample3 jtag
all: sample1 sample2 sample3 jtag

sample1:
	@mkdir -p ${TARGET_DIR}/$@
	@rm -rf ${TARGET_DIR}/$@/*
	${SVDOCGEN} -o ${TARGET_DIR}/$@ -i sample1 And3.sv

sample2:
	@mkdir -p ${TARGET_DIR}/$@
	@rm -rf ${TARGET_DIR}/$@/*
	${SVDOCGEN} -o ${TARGET_DIR}/$@ -i sample1 And3.sv Mux2.sv i/do/not/exist And3.sv

sample3:
	@mkdir -p ${TARGET_DIR}/$@
	@rm -rf ${TARGET_DIR}/$@/*
	${SVDOCGEN} -o ${TARGET_DIR}/$@ ./sample1

jtag:
	@mkdir -p ${TARGET_DIR}/$@
	@rm -rf ${TARGET_DIR}/$@/*
	${SVDOCGEN} -o ${TARGET_DIR}/$@ ./JtagCoreDbg --project-name "CPU Core Debug Interface"
