/* APB bus interface.
 *
 */
interface ApbIf #(parameter WIDTH=32)
(input pclk);
    logic [WIDTH-1:0] paddr;
    logic [WIDTH-1:0] pwdata;
    logic [WIDTH-1:0] prdata;
    logic             penable;
    logic             pwrite;
    logic             psel;
endinterface