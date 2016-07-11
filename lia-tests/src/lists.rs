use lia::runtime::*;

pub fn _lia_call(args: Vec<LiaAny>) -> LiaAny {
    _lia_call!(args)
}

lia! {
    function new(obj) {
        var x = {};
        for (var method : obj.prototype) {
            x[method] = obj.prototype[method];
        }
        @call(obj.prototype.constructor, x);
        return x;
    }

    function list_test() {
        var list = {};
        list.prototype = {
            constructor: function() {
                this.length = 0;
            },
            append: function(x) {
                this[this.length] = x;
                this.length = this.length + 1;
            }
        };

        var x = @new(list);
        x.append(3);
        return x[0];
    }
}

#[test]
fn lia_list_test() {
    let result = call!(list_test());
    cast!(let num: i32 = result);
    assert!(num == 3);
}
