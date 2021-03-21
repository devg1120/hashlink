class Fib {
    static function fibonacci(n: Int): Int {
        return switch n {
            case 0: 1;
            case 1: 1;
            case n: fibonacci(n - 1) + fibonacci(n - 2);
        }
    }

  static public function main():Void {
    var ret = fibonacci(37);
    Sys.println(ret);
  }
}
