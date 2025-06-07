ARCH := loongarch64
MACH :=
TARGET_OS := -linux
ABI := -gnu
LA_PREFIX := ${ARCH}${MACH}${TARGET_OS}${ABI}
LAGCC := ${LA_PREFIX}-gcc
LAOBJDUMP := ${LA_PREFIX}-objdump
QEMU := qemu-${ARCH}
STATIC := -static
PROGRAM_CODE := test2_sub.c
PROGRAM_TARGET := test2_sub
ASSEMBLE_CODE := sub.S

all:
	${LAGCC} ${STATIC} ${PROGRAM_CODE} ${ASSEMBLE_CODE} -o ${PROGRAM_TARGET}

run:
	${QEMU} ${PROGRAM_TARGET}

dump:
	${LAOBJDUMP} -d ${PROGRAM_TARGET} > asm.txt
