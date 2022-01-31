/* Package with enum.
 *
 */
package MyPkg;

    typedef enum bit [1:0] {
        RED, YELLOW, GREEN
    } ETrafficLight;

    typedef struct {
        bit active;
    } State;

    function fn1();
    endfunction

    task run();
    endtask

endpackage