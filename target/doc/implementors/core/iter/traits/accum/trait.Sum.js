(function() {var implementors = {};
implementors["ac_library_rs"] = [{"text":"impl&lt;M:&nbsp;<a class=\"trait\" href=\"ac_library_rs/modint/trait.Modulus.html\" title=\"trait ac_library_rs::modint::Modulus\">Modulus</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;<a class=\"struct\" href=\"ac_library_rs/modint/struct.StaticModInt.html\" title=\"struct ac_library_rs::modint::StaticModInt\">StaticModInt</a>&lt;M&gt;&gt; for <a class=\"struct\" href=\"ac_library_rs/modint/struct.StaticModInt.html\" title=\"struct ac_library_rs::modint::StaticModInt\">StaticModInt</a>&lt;M&gt;","synthetic":false,"types":["ac_library_rs::modint::StaticModInt"]},{"text":"impl&lt;'a, M:&nbsp;<a class=\"trait\" href=\"ac_library_rs/modint/trait.Modulus.html\" title=\"trait ac_library_rs::modint::Modulus\">Modulus</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;&amp;'a <a class=\"struct\" href=\"ac_library_rs/modint/struct.StaticModInt.html\" title=\"struct ac_library_rs::modint::StaticModInt\">StaticModInt</a>&lt;M&gt;&gt; for <a class=\"struct\" href=\"ac_library_rs/modint/struct.StaticModInt.html\" title=\"struct ac_library_rs::modint::StaticModInt\">StaticModInt</a>&lt;M&gt;","synthetic":false,"types":["ac_library_rs::modint::StaticModInt"]},{"text":"impl&lt;I:&nbsp;<a class=\"trait\" href=\"ac_library_rs/modint/trait.Id.html\" title=\"trait ac_library_rs::modint::Id\">Id</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;<a class=\"struct\" href=\"ac_library_rs/modint/struct.DynamicModInt.html\" title=\"struct ac_library_rs::modint::DynamicModInt\">DynamicModInt</a>&lt;I&gt;&gt; for <a class=\"struct\" href=\"ac_library_rs/modint/struct.DynamicModInt.html\" title=\"struct ac_library_rs::modint::DynamicModInt\">DynamicModInt</a>&lt;I&gt;","synthetic":false,"types":["ac_library_rs::modint::DynamicModInt"]},{"text":"impl&lt;'a, I:&nbsp;<a class=\"trait\" href=\"ac_library_rs/modint/trait.Id.html\" title=\"trait ac_library_rs::modint::Id\">Id</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;&amp;'a <a class=\"struct\" href=\"ac_library_rs/modint/struct.DynamicModInt.html\" title=\"struct ac_library_rs::modint::DynamicModInt\">DynamicModInt</a>&lt;I&gt;&gt; for <a class=\"struct\" href=\"ac_library_rs/modint/struct.DynamicModInt.html\" title=\"struct ac_library_rs::modint::DynamicModInt\">DynamicModInt</a>&lt;I&gt;","synthetic":false,"types":["ac_library_rs::modint::DynamicModInt"]}];
implementors["euclid"] = [{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;<a class=\"struct\" href=\"euclid/struct.Angle.html\" title=\"struct euclid::Angle\">Angle</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"euclid/struct.Angle.html\" title=\"struct euclid::Angle\">Angle</a>&lt;T&gt;","synthetic":false,"types":["euclid::angle::Angle"]},{"text":"impl&lt;'a, T:&nbsp;'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> + <a class=\"trait\" href=\"https://docs.rs/num-traits/0.2/num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;&amp;'a <a class=\"struct\" href=\"euclid/struct.Angle.html\" title=\"struct euclid::Angle\">Angle</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"euclid/struct.Angle.html\" title=\"struct euclid::Angle\">Angle</a>&lt;T&gt;","synthetic":false,"types":["euclid::angle::Angle"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"euclid/num/trait.Zero.html\" title=\"trait euclid::num::Zero\">Zero</a>, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;<a class=\"struct\" href=\"euclid/struct.Length.html\" title=\"struct euclid::Length\">Length</a>&lt;T, U&gt;&gt; for <a class=\"struct\" href=\"euclid/struct.Length.html\" title=\"struct euclid::Length\">Length</a>&lt;T, U&gt;","synthetic":false,"types":["euclid::length::Length"]},{"text":"impl&lt;'a, T:&nbsp;'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> + <a class=\"trait\" href=\"euclid/num/trait.Zero.html\" title=\"trait euclid::num::Zero\">Zero</a>, U:&nbsp;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;&amp;'a <a class=\"struct\" href=\"euclid/struct.Length.html\" title=\"struct euclid::Length\">Length</a>&lt;T, U&gt;&gt; for <a class=\"struct\" href=\"euclid/struct.Length.html\" title=\"struct euclid::Length\">Length</a>&lt;T, U&gt;","synthetic":false,"types":["euclid::length::Length"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"euclid/num/trait.Zero.html\" title=\"trait euclid::num::Zero\">Zero</a>, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;<a class=\"struct\" href=\"euclid/struct.Size2D.html\" title=\"struct euclid::Size2D\">Size2D</a>&lt;T, U&gt;&gt; for <a class=\"struct\" href=\"euclid/struct.Size2D.html\" title=\"struct euclid::Size2D\">Size2D</a>&lt;T, U&gt;","synthetic":false,"types":["euclid::size::Size2D"]},{"text":"impl&lt;'a, T:&nbsp;'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> + <a class=\"trait\" href=\"euclid/num/trait.Zero.html\" title=\"trait euclid::num::Zero\">Zero</a>, U:&nbsp;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;&amp;'a <a class=\"struct\" href=\"euclid/struct.Size2D.html\" title=\"struct euclid::Size2D\">Size2D</a>&lt;T, U&gt;&gt; for <a class=\"struct\" href=\"euclid/struct.Size2D.html\" title=\"struct euclid::Size2D\">Size2D</a>&lt;T, U&gt;","synthetic":false,"types":["euclid::size::Size2D"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"euclid/num/trait.Zero.html\" title=\"trait euclid::num::Zero\">Zero</a>, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;<a class=\"struct\" href=\"euclid/struct.Size3D.html\" title=\"struct euclid::Size3D\">Size3D</a>&lt;T, U&gt;&gt; for <a class=\"struct\" href=\"euclid/struct.Size3D.html\" title=\"struct euclid::Size3D\">Size3D</a>&lt;T, U&gt;","synthetic":false,"types":["euclid::size::Size3D"]},{"text":"impl&lt;'a, T:&nbsp;'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> + <a class=\"trait\" href=\"euclid/num/trait.Zero.html\" title=\"trait euclid::num::Zero\">Zero</a>, U:&nbsp;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;&amp;'a <a class=\"struct\" href=\"euclid/struct.Size3D.html\" title=\"struct euclid::Size3D\">Size3D</a>&lt;T, U&gt;&gt; for <a class=\"struct\" href=\"euclid/struct.Size3D.html\" title=\"struct euclid::Size3D\">Size3D</a>&lt;T, U&gt;","synthetic":false,"types":["euclid::size::Size3D"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"euclid/num/trait.Zero.html\" title=\"trait euclid::num::Zero\">Zero</a>, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;<a class=\"struct\" href=\"euclid/struct.Vector2D.html\" title=\"struct euclid::Vector2D\">Vector2D</a>&lt;T, U&gt;&gt; for <a class=\"struct\" href=\"euclid/struct.Vector2D.html\" title=\"struct euclid::Vector2D\">Vector2D</a>&lt;T, U&gt;","synthetic":false,"types":["euclid::vector::Vector2D"]},{"text":"impl&lt;'a, T:&nbsp;'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> + <a class=\"trait\" href=\"euclid/num/trait.Zero.html\" title=\"trait euclid::num::Zero\">Zero</a>, U:&nbsp;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;&amp;'a <a class=\"struct\" href=\"euclid/struct.Vector2D.html\" title=\"struct euclid::Vector2D\">Vector2D</a>&lt;T, U&gt;&gt; for <a class=\"struct\" href=\"euclid/struct.Vector2D.html\" title=\"struct euclid::Vector2D\">Vector2D</a>&lt;T, U&gt;","synthetic":false,"types":["euclid::vector::Vector2D"]},{"text":"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"euclid/num/trait.Zero.html\" title=\"trait euclid::num::Zero\">Zero</a>, U&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;<a class=\"struct\" href=\"euclid/struct.Vector3D.html\" title=\"struct euclid::Vector3D\">Vector3D</a>&lt;T, U&gt;&gt; for <a class=\"struct\" href=\"euclid/struct.Vector3D.html\" title=\"struct euclid::Vector3D\">Vector3D</a>&lt;T, U&gt;","synthetic":false,"types":["euclid::vector::Vector3D"]},{"text":"impl&lt;'a, T:&nbsp;'a + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/arith/trait.Add.html\" title=\"trait core::ops::arith::Add\">Add</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> + <a class=\"trait\" href=\"euclid/num/trait.Zero.html\" title=\"trait euclid::num::Zero\">Zero</a>, U:&nbsp;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/accum/trait.Sum.html\" title=\"trait core::iter::traits::accum::Sum\">Sum</a>&lt;&amp;'a <a class=\"struct\" href=\"euclid/struct.Vector3D.html\" title=\"struct euclid::Vector3D\">Vector3D</a>&lt;T, U&gt;&gt; for <a class=\"struct\" href=\"euclid/struct.Vector3D.html\" title=\"struct euclid::Vector3D\">Vector3D</a>&lt;T, U&gt;","synthetic":false,"types":["euclid::vector::Vector3D"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()