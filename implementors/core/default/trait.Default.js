(function() {var implementors = {};
implementors["atomic_store"] = [{"text":"impl&lt;ParamType:&nbsp;<a class=\"trait\" href=\"https://docs.rs/serde/1.0.166/serde/ser/trait.Serialize.html\" title=\"trait serde::ser::Serialize\">Serialize</a> + <a class=\"trait\" href=\"https://docs.rs/serde/1.0.166/serde/de/trait.DeserializeOwned.html\" title=\"trait serde::de::DeserializeOwned\">DeserializeOwned</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"atomic_store/load_store/struct.BincodeLoadStore.html\" title=\"struct atomic_store::load_store::BincodeLoadStore\">BincodeLoadStore</a>&lt;ParamType&gt;","synthetic":false,"types":["atomic_store::load_store::BincodeLoadStore"]},{"text":"impl&lt;ParamType:&nbsp;CanonicalSerialize + CanonicalDeserialize&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"atomic_store/load_store/struct.ArkLoadStore.html\" title=\"struct atomic_store::load_store::ArkLoadStore\">ArkLoadStore</a>&lt;ParamType&gt;","synthetic":false,"types":["atomic_store::load_store::ArkLoadStore"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.64.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"atomic_store/storage_location/struct.StorageLocation.html\" title=\"struct atomic_store::storage_location::StorageLocation\">StorageLocation</a>","synthetic":false,"types":["atomic_store::storage_location::StorageLocation"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()