/* 3-input AND.
 *
 *
 * <script type="WaveDrom">
 * { assign:[
 *   ["out",
 *     ["&", "in1", "in2", "in3"]
 *   ]
 * ]}
 * </script>
 * 
 */
module And3 #(
    parameter WIDTH = 1
)(
    input  wire [WIDTH-1:0] in1,
    input  wire [WIDTH-1:0] in2,
    input  wire [WIDTH-1:0] in3,
    output wire [WIDTH-1:0] out
);

    assign out = in1 & in2 & in3;

endmodule
