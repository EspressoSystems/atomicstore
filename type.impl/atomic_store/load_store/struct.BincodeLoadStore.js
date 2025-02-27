(function() {
    var type_impls = Object.fromEntries([["atomic_store",[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-BincodeLoadStore%3CParamType%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/atomic_store/load_store.rs.html#25\">Source</a><a href=\"#impl-Debug-for-BincodeLoadStore%3CParamType%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;ParamType: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"atomic_store/load_store/struct.BincodeLoadStore.html\" title=\"struct atomic_store::load_store::BincodeLoadStore\">BincodeLoadStore</a>&lt;ParamType&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/atomic_store/load_store.rs.html#25\">Source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"type\" href=\"https://doc.rust-lang.org/1.85.0/core/fmt/type.Result.html\" title=\"type core::fmt::Result\">Result</a></h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.85.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","atomic_store::load_store::StorageLocationLoadStore"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-BincodeLoadStore%3CParamType%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/atomic_store/load_store.rs.html#41-47\">Source</a><a href=\"#impl-Default-for-BincodeLoadStore%3CParamType%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;ParamType: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.85.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"atomic_store/load_store/struct.BincodeLoadStore.html\" title=\"struct atomic_store::load_store::BincodeLoadStore\">BincodeLoadStore</a>&lt;ParamType&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/atomic_store/load_store.rs.html#42-46\">Source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.85.0/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; Self</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/1.85.0/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","atomic_store::load_store::StorageLocationLoadStore"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-LoadStore-for-BincodeLoadStore%3CParamType%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/atomic_store/load_store.rs.html#30-39\">Source</a><a href=\"#impl-LoadStore-for-BincodeLoadStore%3CParamType%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;ParamType: <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.197/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>&gt; <a class=\"trait\" href=\"atomic_store/load_store/trait.LoadStore.html\" title=\"trait atomic_store::load_store::LoadStore\">LoadStore</a> for <a class=\"struct\" href=\"atomic_store/load_store/struct.BincodeLoadStore.html\" title=\"struct atomic_store::load_store::BincodeLoadStore\">BincodeLoadStore</a>&lt;ParamType&gt;</h3></section></summary><div class=\"impl-items\"><section id=\"associatedtype.ParamType\" class=\"associatedtype trait-impl\"><a class=\"src rightside\" href=\"src/atomic_store/load_store.rs.html#31\">Source</a><a href=\"#associatedtype.ParamType\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"atomic_store/load_store/trait.LoadStore.html#associatedtype.ParamType\" class=\"associatedtype\">ParamType</a> = ParamType</h4></section><section id=\"method.load\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/atomic_store/load_store.rs.html#33-35\">Source</a><a href=\"#method.load\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"atomic_store/load_store/trait.LoadStore.html#tymethod.load\" class=\"fn\">load</a>(&amp;self, stream: &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u8.html\">u8</a>]) -&gt; <a class=\"type\" href=\"atomic_store/type.Result.html\" title=\"type atomic_store::Result\">Result</a>&lt;Self::<a class=\"associatedtype\" href=\"atomic_store/load_store/trait.LoadStore.html#associatedtype.ParamType\" title=\"type atomic_store::load_store::LoadStore::ParamType\">ParamType</a>&gt;</h4></section><section id=\"method.store\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/atomic_store/load_store.rs.html#36-38\">Source</a><a href=\"#method.store\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"atomic_store/load_store/trait.LoadStore.html#tymethod.store\" class=\"fn\">store</a>(&amp;mut self, param: &amp;Self::<a class=\"associatedtype\" href=\"atomic_store/load_store/trait.LoadStore.html#associatedtype.ParamType\" title=\"type atomic_store::load_store::LoadStore::ParamType\">ParamType</a>) -&gt; <a class=\"type\" href=\"atomic_store/type.Result.html\" title=\"type atomic_store::Result\">Result</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.85.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.85.0/std/primitive.u8.html\">u8</a>&gt;&gt;</h4></section></div></details>","LoadStore","atomic_store::load_store::StorageLocationLoadStore"]]]]);
    if (window.register_type_impls) {
        window.register_type_impls(type_impls);
    } else {
        window.pending_type_impls = type_impls;
    }
})()
//{"start":55,"fragment_lengths":[7081]}