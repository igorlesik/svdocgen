# CPU Core Debug Interface

## Main parts of Debug Interface

The Debug Interface (TDI) provides access to debug functionality of CPU Core.
DI serves as a communication and control channel between an external
debugger and Core debug logic.

DI has 3 main components:

- The JTAG Debug Port (JDP).
- The Core Debug Port (CDP).
- The Core Debug Interface (CDI).



```
                                  +---------------------------------------------------------+
                                  |                                                         |
+-----------------------+         |  +---------------------------+     +-------------+      |
|                       |  JTAG   |  |                           |     |             |      |
| JTAG Adapter/Debugger +--------------------+      JDP          |     |    CDP      |      |
|                       |         |  |       |                   +---->+             |      |
+----------+------------+         |  |       |                   |     |             |      |
           ^                      |  |       |    +----------+   |     |             |      |
           | USB                  |  |       |    | JTAG TAP |   |     |             |      |
           |                      |  |       +--->+          |   |     |             |      |
+----------+------------+         |  |            +----------+   |     |             |      |
|                       |         |  +---------------------------+     +-------------+      |
|  Host debugger        |         |                                           |             |
|  OpenOCD + GDB        |         |                                           |             |
|                       |         |   DAP - Debug Access Port                 |             |
+-----------------------+         |                                           |             |
                                  +-------------------------------------------|-------------+
                                                                              |
                                    DAP is APB Master                         | APB bus
                                                                              v
                                                                  +-----------------------+
                                                                  |  Core Debug Interface |
                                                                  +-----------------------+
                                                                    one per Core, APB Slave
```

Debug Access Port (DAP) is made of JDP and CDP.
There is a single instance of DAP on the chip.
An external debugger is connected to the JTAG pins of DAP;
DAP is APB Master and performs APB writes/reads to/from CDIs
that are APB Slaves.
Thus, DAP provides external debugger with a standard interface to access
Core debug facilities.



## JTAG Debug Port (JDP)

JTAG Debug Port (JDP) handles communication with an external JTAG Debugger/Adapter.
In its turn, JDP conveys the external requests to Core Debug Port (CDP).

To use JTAG, a host/debugger is connected to the JDP JTAG signals
(TMS, TCK, TDI, TDO, etc.) through some kind of JTAG adapter (also called dongle, probe).
The adapter connects to the host using some interface such
as USB, PCI, Ethernet, and so forth.
The purpose of the adapter is to translate host/debugger commands
into the JTAG signals.

Test Access Port (TAP) is JTAG interface; JTAG pins are TAP pins.
The JTAG signals/pins, and TAP connector pins, are:

- TDI: Test Data In
- TDO: Test Data Out
- TCK: Test Clock
- TMS: Test Mode Select
- TRST: Test Reset (optional)

### TAP

JDP contains JTAG Test Access Port (TAP) that is responsible for the
control and communication.
The host communicates with the JDP TAP by manipulating TMS and TDI in conjunction with TCK,
and reading results through TDO.

The JTAG Test Access Point (TAP) is composed of:

- the TAP controller,
- an instruction register,
- several test data registers,
- some glue-logic.

The Test Access Port (TAP) controller contains the testing state machine,
and is responsible for interpreting the TCK and TMS signals;
the state machine selects proper TAP register for the communication.

```

                         +-----+           Data registers
+------------+           |     |
|            |           | SEL |           +----------------+
|            +---------->+     +----+------+                |
|   Select   |           +--+--+    |      +-+--------------+
|     MUX    |              ^       +----->  |
|            |              |                |  +----------------+
|            |     +----------------------+  |  |                |
|            +-----> Instruction register |  |  +-+--------------+
+---+---+----+     +-----+------+---------+  |    |
    ^   ^                |      |            |    |  +----------------+
    |   | sel            |      |            |    |  |                |
    |   |                |      |            |    |  +-+--------------+
    |   |                |      |            |    |    |
    |   |                |      |            |    |    |
    |   |                |      |   sel  +---v----v----v----+
    |   |                |      +------->+    Select/MUX    |
    |   |                |               +---------+--------+
    |   |                |                         |
    |   |                +-------------------+     |
    |   |                                    |     |
    |   |                                    |     |
    |   |                                    v     v
    |   |         +-----------------+     +--+-----+-----+
    |   +---------+ TAP Controller  +---->+  Select/MUX  |
    |             +---+-------+-----+     +------+-------+
    |                 ^       ^                  |
    |                 |       |                  |
    |                 |       |                  |
    +                 +       +                  v
   TDI               TCK     TMS                TDO

```

### TAP State Machine

![TAP State Machine](
https://upload.wikimedia.org/wikipedia/commons/thumb/1/1a/JTAG_TAP_Controller_State_Diagram.svg/563px-JTAG_TAP_Controller_State_Diagram.svg.png)

The Instruction Register (IR) path is used for loading instructions.
The Data Register (DR) path is used for reading/writing data from/to data registers.

### JTAG TAP Instruction Register (IR)

Most JTAG _instructions_ can broadly be described as connecting different data registers to the TDI/TDO path.
The `BYPASS` instruction connects TDI directly to TDO through a 1-bit shift register,
the `IDCODE` instruction connects the identification code register to TDO.
Both `BYPASS` and `IDCODE` are required by the JTAG standard.

JTAG IR is using 8-bit encoding.
Only 3 registers: BYPASS, IDCODE and CDPACC are currently implemented.
Rest is reserved for future use.

```Verilog
    localparam INSN_EXTEST         = 8'b0000_0000; // reserved
    localparam INSN_SAMPLE_PRELOAD = 8'b0000_0001; // reserved
    localparam INSN_IDCODE         = 8'b0000_0010; // required by the standard
    localparam INSN_DEBUG          = 8'b0000_1000; // reserved
    localparam INSN_MBIST          = 8'b0000_1001; // reserved
    localparam INSN_JDPACC         = 8'b0000_0100; // reserved
    localparam INSN_CDPACC         = 8'b0000_0101; // implemented
    localparam INSN_BYPASS         = 8'b1111_1111; // all 1's required by the standard
```

## Architecture specific JTAG instructions and registers.

JDPACC (JTAG Debug Port Access) and CDPACC (Core Debug Port Access) are the instructions/registers
used to pass commands through to the associated ports.
By setting different values in the JDPACC or CDPACC data registers,
the debugger can execute different operations,
generally interacting with the registers of the JDP and CDP.

JDPACC (JTAG Debug Port Access) and
CDPACC (Core Debug Port Access) instructions encoding:

```Verilog
    localparam INSN_JDPACC         = 8'b0000_0100;
    localparam INSN_CDPACC         = 8'b0000_0101;
```

The debugger uses the JTAG interface to execute instructions JDPACC, CDPACC
by going through the TAP state machine, then the instructions take the data
and load it into the JDPACC, CDPACC registers, and depending on the data,
different registers within the JDP or CDP are accessed, providing the desired
comminication to the Core Debug logic.

Note that JDP registers are currently **NOT** implemented and considered reserved:

- DPIDR (Debug Port ID Register): Architecture chip ID, number of cores.
- ABORT: force CDP transaction abort.

CDP registers:

- SELECT: holds currently selected Core number.
- TADDR (Transfer Address Register): holds the address for the next access to
  memory-mapped Core Debug register.
- DTR (Data Transfer Register): write/read to/from this register trigger
  APB write/read transaction with address equal value of TADDR and
  APB Slave selected according to the value of SELECT register.

## JDP and CDP access register operations

The TAP state machine by design does not allow arbitrary transitions.
Basically, DR states change in the following order:

1. Select-DR-Scan: start new cycle.
2. Capture-DR
3. Shift-DR
4. Update-DR
5. Exit the cycle and go back to Select-DR-Scan.

We map the access operations to the Capture-Shift-Update sequence:

- Capture-DR: move last read ACK and data into DR.
- Shift-DR: shift read data out, shift write data in.
- Update-DR: trigger the update using shifted-in data.

In the **Capture-DR** state, the result of the previous transaction,
if any, is returned, together with a 4-bit ACK response.

<script type="WaveDrom">
{ reg: [
  {bits: 4, name: 'ACK', attr: ''},
  {bits: 32, name: 'DATA', attr: 'Read Result'}
], 
  config:{bits: 36}
}
</script>

In the **Shift-DR** state, ACK[3:0] is shifted out first, then 32 bits
of "Read Result" data is shifted out.

As the returned data is shifted out to `TDO`,
new data is shifted in from `TDI`.

<script type="WaveDrom">
{ reg: [
  {bits: 4, name: 'DATA', attr: 'Data[3:0]'},
  {bits: 32, name: 'DATA', attr: 'Data[35:4]'}
], 
  config:{bits: 36}
}
</script>

If the response indicated by ACK[3:0] is OK/FAULT (not WAIT),
the previous transaction has completed.
An OK/FAULT response is followed by an **Update-DR** operation
to fulfill the read or write request that is formed
by the values that were shifted into the scan chain.
For write requests, WR is having a value of 0b1,
the value in DATAIN[31:0] is written to the
register selected by ADDR[2:0].
For read requests, WR is having a value of 0b0,
the value in DATAIN[31:0] is ignored,
to be read register is selected by ADDR[2:0].

<script type="WaveDrom">
{ reg: [
  {bits: 1, name: 'WR', attr: ''},
  {bits: 3, name: 'ADDR', attr: 'select'},
  {bits: 32, name: 'DATA', attr: 'Data[35:4]'}
], 
  config:{bits: 36}
}
</script>

## Core Debug Port (CDP)

CDP takes requests from JDP and sends it over memory interface (APB bus) to Core Debug Interface (CDI).

First, external JTAG debugger shall set IR to the value CDPACC, this makes CDPACC Data Register selected
as current DR.

Second, debugger selects which CDP register it intends to access
by defining bits CDPACC[3:1]:

```Verilog
    localparam CDP_OP_SELECT = 3'b000,
               CDP_OP_TADDR  = 3'b001,
               CDP_OP_DTR    = 3'b010;
```

Bit CDPACC[0] defines access type, 1 - write, 0 - read.

Bits CDPACC[35:4] hold the data.

Writing to TDR register triggers APB transaction with
following parameters:

- APB Slave is selected based on the value of SELECT register.
- APB Address (5 bits) is value of TADDR register.
- APB Data is (32 bits) value of DTR register

Note that the debugger has to correctly change TAP FSM states with TMS signal
while manipulating CDP registers.

Below is an pseudo code example of writing number 2 into TADDR register.

```Verilog
    $display("%t Set state=Shift-IR", $time);
    _jtag.go_shift_ir();
    $display("%t Set IR=CDPACC", $time);
    _jtag.do_ir_cdpacc();
 
    _jtag.go_exit_ir_to_idle();

    // Set state=Shift-DR and shift in 36 bits of data=TADDR-value
    $display("%t Set state=Shift-DR", $time);
    _jtag.go_idle_to_shift_dr();

    $display("%t shift-in TADDR+W", $time);
    _jtag.tick4(2'b01, 2'b01, 2'b00, 2'b00); // [0] 1=W [3:1] 001==TADDR
    $display("%t shift-in TADDR val", $time);
    _jtag.do_shiftin_int32(32'h2);

    // Trigger APB transaction
    $display("%t Set state=Update-DR", $time);
    _jtag.go_exit1_to_update_dr();
```


### SELECT Register

SELECT register is Write Only register that holds value of last selected
Core ID. The value is persistent and is used till next write to SELECT register.


<script type="WaveDrom">
{ reg: [
  {bits: 12, name: 'COREID', attr: 'Core ID'}
], 
  config:{bits: 32}
}
</script>


### TADDR Register

TADDR register is Write Only register that holds value of last selected
Debug register address.
The value is persistent and is used till next write to TADDR register.


<script type="WaveDrom">
{ reg: [
  {bits: 5, name: 'ADDR', attr: 'Reg offset'}
], 
  config:{bits: 32}
}
</script>


### DTR Register

Write to DTR register triggers APB write transaction to the corresponding
Debug Interface register that in its own turn triggers an action of Core Debug logic.

Read from DTR register triggers APB read transaction from the corresponding
Debug Interface register that in its own turn triggers reading of some Core register.

<script type="WaveDrom">
{ reg: [
  {bits: 32, name: 'DATA', attr: ''}
], 
  config:{bits: 32}
}
</script>


## Core Debug Interface registers

Debug Interface registers.

| Offset | Mnemonic | Access | Description
| :----- | :------- | :----: |:----------------------------
| 0      | DBGSC    |   W    | Debug Status and Control Register
| 1      | DRUNCTRL |   W    | Debug Run Control Register
| 2      | ITR0     |   W    | Instruction Transfer Register 0
| 3      | ITR1     |   W    | Instruction Transfer Register 1
| 4      | ITR2     |   W    | Instruction Transfer Register 2
| 5      | ITR3     |   W    | Writing to ITR3 triggers ITR execution
| 6      | DTR_HI   |   RW   | Data Transfer Register upper 32 bits
| 7      | DTR_LO   |   RW   | Data Transfer Register lower 32 bits


### DBGSC - Debug Status and Control Register

The DBGSC is a RW register that contains status
and control information about the debug unit.

<script type="WaveDrom">
{ reg: [
  {bits: 1, name: 'H', attr: ''},
  {bits: 1, name: 'R', attr: ''},
  {bits: 1, name: 'DE', attr: ''},
  {bits: 1, name: 'EX', attr: ''},
  {bits: 1, name: 'IC', attr: ''}
], 
  config:{bits: 32}
}
</script>



| Bits    | Field      | RW   | Function
| ------- | ---------- | ---- | ---------------------------------------------
| [0]     | H          | R    | Core halted bit:<br>0 = The processor is in normal state. This is the reset value.<br>1 = The processor is in debug state.<br>The debugger can poll this bit to determine when the processor has entered debug state.
| [1]     | R          | R    | Core restarted bit
| [2]     | DE         | RW   | The Halting debug-mode enable bit. This is a read/write bit. 0 = Halting debug-mode disabled, reset value. 1 = Halting debug-mode enabled.
| [3]     | EX         | RW   | If this bit is set to 1 and an ITR write succeeds, the processor fetches an instruction from the ITR for execution.
| [4]     | INSNCOMPL  | R    | 0 = the processor is currently executing an instruction fetched from the ITR. 1 = the processor is not currently executing an instruction fetched from the ITR Register.




### DRUNCTRL - Debug Run Control Register

The DRUNCTRL is Write-Only registers which facilitates
requesting the processor to enter or leave debug state
and single step an instruction.

<script type="WaveDrom">
{ reg: [
  {bits: 1, name: 'H', attr: ''},
  {bits: 1, name: 'R', attr: ''},
  {bits: 1, name: 'ST', attr: ''}
], 
  config:{bits: 32}
}
</script>

Table: DRUNCTRL bits.

| Bits    | Field       | Function
| ------- | ----------- | ---------------------------------------------
| [0]     | H           | Halt request. Writing a 1 to this bit triggers a halting debug event, that is, a request that the processor enters debug state.
| [1]     | R           | Restart request. Writing a 1 to this bit requests that the processor leaves debug state. This request is held until the processor exits debug state. The debugger must poll DBGSC[1] to determine when this request succeeds.
| [2]     | STEP        | Execute one instruction. Core makes Fetch Group with one instruction (or u-ops) and NOPs from fetched FG. Core gets restarted and then halted again upon the FG retire event.



### ITR - Instruction Transfer Register

The ITR registers enable the external debugger with the functionality
to feed instructions into the core
for execution while in debug state.
The ITRn is a write-only register.


<script type="WaveDrom">
{ reg: [
  {bits: 32,  name: 'Instruction', attr: ['Instruction opcode']}
], 
  config:{bits: 32}
}
</script>

Writing to ITR3 triggers ITR execution when Core is in Debug Mode
DBGSC[0]==1 and ITR execution is enabled DBGSC[3]==1.

With 4 instructions external debugger may use XIMM prefix
and generally form a complete Fetch Group to be executed.
Unused instructions should be NOPs.


### DTR - Data Transfer Register

Data Transfer Register is 64 bit and implewmented as 2 32 bits registers:
DTR_HI and DTR_LO.
Writing to DTR_LO triggers writing to `EDBGDTR`
system registers the content of both DTR_HI and DTR_LO.

Reading of DTR_HI and DTR_LO can be done in any order.

<script type="WaveDrom">
{ reg: [
  {bits: 32,  name: 'DATA', attr: ['32 bits Data Transfer Register']}
], 
  config:{bits: 32}
}
</script>

#### Core special register EDBGDTR for debug data transfer

**All** data transfer between external debugger and Core happens via
special register EDBGDTR (External Debug Data Transfer Register).
Register `EDBGDTR`
on one side is visible to Core as a special register,
on other side it can be accesses by CDI (Core Debug Interface).

On Core side to write/read to EDBGDTR Instructions `mts`/`mfs`
get executed via ITR execution.

Instruction `mts sedbgdtr, <SOURCE>` is used to read Core architecture state variables.

Instruction `mfs sedbgdtr, <DEST>` is used to set Core architecture state variables.


## Exceptions in debug state

When external debugger is connected, exceptions are handled as follows:

- halt Core vs call interrupt handler (we usually do not care
  about timer interrupt and etc, Page Fault normally should be handled by the handler)
- halt Core on break-point and watch-point
- during ITR while halted


## Debug on Reset

A Debug-on-Reset event allows the external debugger to take control
of the CPU immediately after reset.
To enable this event, set the Debug on Reset (DRE) bit in the
External Debug Execution Control Register (EDECR).
When the external debugger sets the DRE bit, the Debug-on-Reset debug event is generated
on a reset of the CPU.
The CPU enters Debug state and gives control to the external debugger,
before the execution of the first instruction after reset.
The reset can be either a Warm reset or a Cold reset of the CPU.
To exit Debug state, the external debugger asserts a restart request to the processor.



## Examples of using the debug functionality

The debugger examples use a few of pseudo-functions such as the following:

```c
// Read 32 bits wide DI register, or lower 32 bits of 64 bits register.
uint32_t readDbgReg(unsigned int offset);

// Read 64 bits wide DI register.
uint64_t readDbgReg64(unsigned int offset);

// Write 32 bits wide DI register.
void writeDbgReg(unsigned int offset, uint32_t val);

// Write 64 bits wide DI register.
void writeDbgReg64(unsigned int offset, uint64_t val);

bool isBitSet(uint32_t regval, uint8_t bit);
```


### Execute an instruction through the ITR

The main tool during the debugging is
executing an instruction through the ITR.
With it we can get and update Core architectural state.

Before the debugger can force the processor to execute any instruction,
it must enable this feature through DBGSC[3].

```c
void executeInsn(uint32_t insn_opcode[4])
{
    // Wait ITR completion.
    // Poll DBGSC until bit INSNCOMPL is set to 1.
    while (!isBitSet(readDbgReg(DBGSC), DBGSC_INSNCOMPL)) {
        // wait
    }
    
    // Write the opcode to the ITR.
    writeDbgReg(ITR0, insn_opcode[0]);
    writeDbgReg(ITR1, insn_opcode[1]);
    writeDbgReg(ITR2, insn_opcode[2]);
    writeDbgReg(ITR3, insn_opcode[3]);

    // Wait ITR completion.
    // Poll DBGSC until bit INSNCOMPL is set to 1.
    while (!isBitSet(readDbgReg(DBGSC), DBGSC_INSNCOMPL)) {
        // wait
    }
}
```


### Read General Purpose register value

```c
uint64_t readGPReg(uint8_t regId)
{
    uint32_t insn_mts_edbgdtr = encodeInsn("mts edbgdtr, regId");

    executeInsn(insn_mts_edbgdtr);

    // DISCUSS if we need DTRTX
    // Poll DBGSC until bit DTRTX is set to 1.
    //while (!isBitSet(readDbgReg(DBGSC), DBGSC_DTRTX)) {
        // wait
    //}

    uint64_t gprVal = readDbgReg64(DTR);
    
    return gprVal;
}
```


### Write General Purpose register value


```c
void writeGPReg(uint8_t regId, uint64_t val)
{
    uint32_t insn_mfs_edbgdtr = encodeInsn("mfs edbgdtr, regId");

    // DISCUSS if we need DTRRX
    // Poll DBGSC until bit DTRRX is cleared to 0.
    //while (isBitSet(readDbgReg(DBGSC), DBGSC_DTRRX)) {
        // wait
    //}

    writeDbgReg64(DTR, val);

    executeInsn(insn_mfs_edbgdtr);
}
```

### Read Floating-point register

```c
uint64_t readFPReg64(uint8_t regId)
{
    // Save r10
    uint64_t r10 = readGPReg(10);

    // Move FP reg value to r10
    executeInsn("fmov r10, regId");

    uint64_t fpRegVal = readGPReg(10);

    // Restore r10
    writeGPReg(10, r10);

    return fpRegVal;
}
```


### Write Floating-point register

```c
void writeFPReg(uint8_t regId, double val)
{
    // Save r10
    uint64_t r10 = readGPReg(10);

    // Write 64-bit double to r10
    writeGPReg(10, asUInt64(val));

    // Move r10 to FP reg
    executeInsn("fmov regId, r10");

    // Restore r10
    writeGPReg(10, r10);
}
```


### Read Vector register

With a function like `readVRegElement` one can read Vector register
one element per time.

```c
uint64_t readVRegElement(uint8_t regId, uint8_t pos)
{
    // Save r10
    uint64_t r10 = readGPReg(10);

    // Move Vector reg element value to r10
    executeInsn("vxtr r10, regId, pos");

    uint64_t val = readGPReg(10);

    // Restore r10
    writeGPReg(10, r10);

    return val;
}
```


### Write Vector register

With a function like `writeVRegElement` one can write Vector register
one element per time.

```c
void writeVRegElement(uint8_t vregId, uint8_t pos, uint64_t val)
{
    // Save r10
    uint64_t r10 = readGPReg(10);

    // Write val to r10
    writeGPReg(10, val);

    // Move r10 to FP reg
    executeInsn("vins vregId, r10, pos");

    // Restore r10
    writeGPReg(10, r10);
}
```

### Read Vector Mask register


```c
uint64_t readVMaskReg(uint8_t mregId, bool readLow64)
{
    // Save r10
    uint64_t r10 = readGPReg(10);

    uint64_t val = 0;

    // Move Mask reg value to r10
    if (readLow64) {
        executeInsn("pmov r10, mregId");
        val = readGPReg(10);
    }
    else {
        uint64_t low64 = readVMask(mregId, /*low=*/true);
        executeInsn("pshr mregId, mregId, 64");
        executeInsn("pmov r10, mregId");
        val = readGPReg(10);
        executeInsn("pshl mregId, mregId, 64");
        writeGPReg(10, low64);
        executeInsn("pins mregId, r10, 1");//TODO we do not have pins yet
    }

    // Restore r10
    writeGPReg(10, r10);

    return val;
}
```

### Write Vector Mask register

```c
void writeVMaskReg(uint8_t mregId, uint64_t val, bool writeLow64)
{
    // Save r10
    uint64_t r10 = readGPReg(10);

    // Write val to r10
    writeGPReg(10, val);

    // Move r10 to Mask reg
    if (writeLow64) {
        executeInsn("pmov mregId, r10");
    }
    else {
        // Save lower 64 bits of the mask reg
        executeInsn("pmov mregId, r10");
        // shift left mask reg and then insert saved lower 64 bits
    }

    // Restore r10
    writeGPReg(10, r10);
}
```

### Read Special register

```c
uint64_t readSysReg(uint8_t sregId)
{
    // Save r10
    uint64_t r10 = readGPReg(10);

    executeInsn("mfs r10, sregId");

    uint64_t val = readGPReg(10);

    // Restore r10
    writeGPReg(10, r10);

    return val;
}
```


### Write Special register

```c
void writeSysReg(uint8_t sregId, uint64_t val)
{
    // Save r10
    uint64_t r10 = readGPReg(10);

    // Write val to r10
    writeGPReg(10, val);

    // Move r10 to the Sys reg
    executeInsn("mts sregId, r10");

    // Restore r10
    writeGPReg(10, r10);
}
```


### Read Program Counter (PC)

Executing `la rX, 0` via ITR loads current PC into `rX`.

```c
uint64_t getPC()
{
    // Save r10
    uint64_t r10 = readGPReg(10);

    executeInsn("la r10, 0");

    uint64_t val = readGPReg(10);

    // Restore r10
    writeGPReg(10, r10);

    return val;
}
```


### Set next Front End fetching address (Program Counter)

Executing `jmp` or `jmpr` via ITR redirects Front End to the
required fetch address.

```c
void setPC(uint64_t addr)
{
    // Save r10
    uint64_t r10 = readGPReg(10);

    // Write val to r10
    writeGPReg(10, addr);

    // Jump
    executeInsn("jmpr r10");

    // Restore r10
    writeGPReg(10, r10);
}
```


### Programming breakpoints


Programming a breakpoint is made via calling function `writeSysReg()`
with `regId` corresponding to `KBPT`. 


```c
void setBreakpoint(uint8_t bpNum, uint64_t address, uint8_t enable)
{
    uint64_t val = (enable & 1) | (address << 2);

    switch (bpNum) {
        0: writeSysReg(KBPT0, val); break;
        1: writeSysReg(KBPT1, val); break;
        default: assert(!"Illegal break-point number");
    }
}
```

### Programming watchpoints

Programming a watchpoint is made via calling function `writeSysReg()`
with `regId` corresponding to
`KWPT`
and `KWPCTR`.


```c
void setWatchpoint(uint8_t wpNum, uint64_t addr, uint64_t ctrl)
{
    switch (wpNum) {
        0: writeSysReg(KWPT0, addr); break;
        1: writeSysReg(KWPT1, addr); break;
        2: writeSysReg(KWPT2, addr); break;
        3: writeSysReg(KWPT3, addr); break;
        default: assert(!"Illegal watch-point number");
    }

    writeSysReg(KWPTR, ctrl);
}
```

### Stepping using breakpoint

```c
void stepWithBP()
{
    // disable BP at current PC, set BP at next PC
    setBreakpoint(0, getPC() + 4, true);

    // restart by writing 1 to bit DRUNCTRL.RESTART
    writeDbgReg(DRUNCTRL, DRUNCTRL_RESTART);
}
```


### Stepping using STEP command


```c
void step()
{
    // By writing 1 to bit DRUNCTRL.STEP tell Core we want to
    // execute only one instruction.
    writeDbgReg(DRUNCTRL, DRUNCTRL_STEP);

    while (!isBitSet(readDbgReg(DBGSC), DBGSC_HALTED)) {
        // wait till FG retires and Core gets halted again
    }
}
```

### Read memory

**DISCUSS** memory exceptions during LOAD

```c
uint64_t loadMem(uint64_t addr)
{
    // Save r10, r11
    uint64_t r10 = readGPReg(10);
    uint64_t r11 = readGPReg(11);

    writeGPReg(11, addr);

    executeInsn("ld r10, [r11]");

    uint64_t val = readGPReg(10);

    // Restore r10, r11
    writeGPReg(10, r10);
    writeGPReg(11, r11);

    return val;
}
```

### Write memory

```c
void storeMem(uint64_t addr, uint64_t val)
{
    // Save r10, r11
    uint64_t r10 = readGPReg(10);
    uint64_t r11 = readGPReg(11);

    writeGPReg(10, val);
    writeGPReg(11, addr);

    executeInsn("st [r11], r10");

    // Restore r10, r11
    writeGPReg(10, r10);
    writeGPReg(11, r11);
}
```


## OpenOCD with SEGGER J-Link probe on Ubuntu

### Ubuntu as host PC OS

We are going to use Ubuntu Linux for the host PC
connected to SEGGER J-Link probe.

Here is Ubuntu version used for all following examples:

```terminal
$lsb_release -a
Distributor ID: Ubuntu
Description:    Ubuntu 20.04.1 LTS
Release:        20.04
Codename:       focal
```


### Do NOT use SEGGER USB driver and tools

SEGGER J-Link probe has micro-controller inside that makes
it "intelligent" device. SEGGERS USB driver and the host tools
rely on this intelligence.
For example, J-Link probe can act as GDB Server without any help
from other SW.
However, this advance feature only works in case of supported targets.
Since J-Link probe firmware and tools do not know about Prodigy CPU,
we can NOT use this feature at this point; instead we are going
to use J-Link as "dumb" JTAG probe under control of OpenOCD.


### Install libusb

If you already have SEGGER USB driver installed on your system,
we recommend to uninstall it.

To install libusb use following command:

```terminal
sudo apt-get install libusb-1.0-0 libusb-1.0-0-dev
```

### Install OS pre-packaged OpenOCD

To install OS pre-packeged OpenOCD use following command:

```terminal
sudo apt-get install openocd
```

After installing OpenOCD check that it was installed properly:

```terminal
$openocd --version
Open On-Chip Debugger 0.10.0
```

If J-Link probe is not connected to the PC,
OpenOCD shall report that the device could not be found:

```terminal
$openocd -f interface/jlink.cfg
Error: No J-Link device found.
```

Connect J-Link probe to the PC with USB cable and run following
command to confirm that OpenOCD recognizes the probe:

```terminal
$openocd -f interface/jlink.cfg
Info: J-Link V11 compiled Sep 21 2020 17:00:29
Info: Hardware version: 11.00
```

At this point you should be ready to use J-Link probe with OpenOCD.
To see the list of available OpenOCD CLI commands for J-Link
type:

```terminal
$openocd -f interface/jlink.cfg -c jlink
```



