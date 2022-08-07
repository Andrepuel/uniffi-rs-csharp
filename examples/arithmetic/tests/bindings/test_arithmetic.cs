using System.Diagnostics;
using static Arithmetic;

// try {
//     Add(18446744073709551615, 1);
//     Debug.Assert(false, "Should have thrown a IntegerOverflow exception!");
// } catch (ArithmeticError.IntegerOverflow e) {
//     // It's okay!
// }

Debug.Assert(Add(2, 4) == 6);
Debug.Assert(Add(4, 8) == 12);

// try {
//     Sub(0, 1);
//     Debug.Assert(false, "Should have thrown a IntegerOverflow exception!");
// } catch(ArithmeticError.IntegerOverflow e) {
//     // It's okay!
// }

Debug.Assert(Sub(4, 2) == 2);
Debug.Assert(Sub(8, 4) == 4);

Debug.Assert(Div(8, 4) == 2);

// try {
//     Div(8, 0);
//     Debug.Assert(false, "Should have panicked when dividing by zero");
// } catch(InternalError e) {
//     // It's okay!
// }

Debug.Assert(Equal(2, 2));
Debug.Assert(Equal(4, 4));

Debug.Assert(!Equal(2, 4));
Debug.Assert(!Equal(4, 8));
