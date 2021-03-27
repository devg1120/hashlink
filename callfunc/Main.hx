
import callfunc.Callfunc;

class Main {
    static public function main(): Void {

function myHaxeCallback(a:Int, b:Int):Int {
    return b - a;
}

var ffi = Callfunc.instance();
var callbackDef = ffi.wrapCallback(
    myHaxeCallback,
    [DataType.SInt32, DataType.SInt32],
    DataType.SInt32
);

library.define("do_something", [DataType.Pointer]);
library.s.do_something.call(callbackDef.pointer);

}
}
