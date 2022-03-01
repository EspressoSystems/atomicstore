(function() {var implementors = {};
implementors["atomic_store"] = [{"text":"impl&lt;ResourceAdaptor&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/append_log/struct.AppendLog.html\" title=\"struct atomic_store::append_log::AppendLog\">AppendLog</a>&lt;ResourceAdaptor&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ResourceAdaptor: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::append_log::AppendLog"]},{"text":"impl&lt;'a, ResourceAdaptor&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/append_log/struct.Iter.html\" title=\"struct atomic_store::append_log::Iter\">Iter</a>&lt;'a, ResourceAdaptor&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ResourceAdaptor: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::append_log::Iter"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/atomic_store/struct.AtomicStoreFileContents.html\" title=\"struct atomic_store::atomic_store::AtomicStoreFileContents\">AtomicStoreFileContents</a>","synthetic":true,"types":["atomic_store::atomic_store::AtomicStoreFileContents"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/atomic_store/struct.AtomicStoreLoader.html\" title=\"struct atomic_store::atomic_store::AtomicStoreLoader\">AtomicStoreLoader</a>","synthetic":true,"types":["atomic_store::atomic_store::AtomicStoreLoader"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/atomic_store/struct.AtomicStore.html\" title=\"struct atomic_store::atomic_store::AtomicStore\">AtomicStore</a>","synthetic":true,"types":["atomic_store::atomic_store::AtomicStore"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"atomic_store/error/enum.PersistenceError.html\" title=\"enum atomic_store::error::PersistenceError\">PersistenceError</a>","synthetic":true,"types":["atomic_store::error::PersistenceError"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.FailedToResolvePath.html\" title=\"struct atomic_store::error::FailedToResolvePath\">FailedToResolvePath</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::FailedToResolvePath"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.FailedToFindExpectedResource.html\" title=\"struct atomic_store::error::FailedToFindExpectedResource\">FailedToFindExpectedResource</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::FailedToFindExpectedResource"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.InvalidPathToFile.html\" title=\"struct atomic_store::error::InvalidPathToFile\">InvalidPathToFile</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::InvalidPathToFile"]},{"text":"impl&lt;__T0, __T1&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.InvalidFileContents.html\" title=\"struct atomic_store::error::InvalidFileContents\">InvalidFileContents</a>&lt;__T0, __T1&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;__T1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::InvalidFileContents"]},{"text":"impl&lt;__T0, __T1&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.FailedToWriteToFile.html\" title=\"struct atomic_store::error::FailedToWriteToFile\">FailedToWriteToFile</a>&lt;__T0, __T1&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;__T1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::FailedToWriteToFile"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.DuplicateResourceKey.html\" title=\"struct atomic_store::error::DuplicateResourceKey\">DuplicateResourceKey</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::DuplicateResourceKey"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.ResourceFormatInconsistent.html\" title=\"struct atomic_store::error::ResourceFormatInconsistent\">ResourceFormatInconsistent</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::ResourceFormatInconsistent"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.FeatureNotYetImplemented.html\" title=\"struct atomic_store::error::FeatureNotYetImplemented\">FeatureNotYetImplemented</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::FeatureNotYetImplemented"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.StdIoDirOpsError.html\" title=\"struct atomic_store::error::StdIoDirOpsError\">StdIoDirOpsError</a>","synthetic":true,"types":["atomic_store::error::StdIoDirOpsError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.StdIoOpenError.html\" title=\"struct atomic_store::error::StdIoOpenError\">StdIoOpenError</a>","synthetic":true,"types":["atomic_store::error::StdIoOpenError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.StdIoSeekError.html\" title=\"struct atomic_store::error::StdIoSeekError\">StdIoSeekError</a>","synthetic":true,"types":["atomic_store::error::StdIoSeekError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.StdIoWriteError.html\" title=\"struct atomic_store::error::StdIoWriteError\">StdIoWriteError</a>","synthetic":true,"types":["atomic_store::error::StdIoWriteError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.StdIoReadError.html\" title=\"struct atomic_store::error::StdIoReadError\">StdIoReadError</a>","synthetic":true,"types":["atomic_store::error::StdIoReadError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.BincodeSerError.html\" title=\"struct atomic_store::error::BincodeSerError\">BincodeSerError</a>","synthetic":true,"types":["atomic_store::error::BincodeSerError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.BincodeDeError.html\" title=\"struct atomic_store::error::BincodeDeError\">BincodeDeError</a>","synthetic":true,"types":["atomic_store::error::BincodeDeError"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.ArkSerError.html\" title=\"struct atomic_store::error::ArkSerError\">ArkSerError</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::ArkSerError"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.ArkDeError.html\" title=\"struct atomic_store::error::ArkDeError\">ArkDeError</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::ArkDeError"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.OtherStoreError.html\" title=\"struct atomic_store::error::OtherStoreError\">OtherStoreError</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::OtherStoreError"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.OtherLoadError.html\" title=\"struct atomic_store::error::OtherLoadError\">OtherLoadError</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::OtherLoadError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.GlobSyntax.html\" title=\"struct atomic_store::error::GlobSyntax\">GlobSyntax</a>","synthetic":true,"types":["atomic_store::error::GlobSyntax"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.GlobRuntime.html\" title=\"struct atomic_store::error::GlobRuntime\">GlobRuntime</a>","synthetic":true,"types":["atomic_store::error::GlobRuntime"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/error/struct.SyncPoisonError.html\" title=\"struct atomic_store::error::SyncPoisonError\">SyncPoisonError</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::SyncPoisonError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/fixed_append_log/struct.IndexContents.html\" title=\"struct atomic_store::fixed_append_log::IndexContents\">IndexContents</a>","synthetic":true,"types":["atomic_store::fixed_append_log::IndexContents"]},{"text":"impl&lt;ResourceAdaptor&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/fixed_append_log/struct.FixedAppendLog.html\" title=\"struct atomic_store::fixed_append_log::FixedAppendLog\">FixedAppendLog</a>&lt;ResourceAdaptor&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ResourceAdaptor: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::fixed_append_log::FixedAppendLog"]},{"text":"impl&lt;'a, ResourceAdaptor&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/fixed_append_log/struct.Iter.html\" title=\"struct atomic_store::fixed_append_log::Iter\">Iter</a>&lt;'a, ResourceAdaptor&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ResourceAdaptor: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::fixed_append_log::Iter"]},{"text":"impl&lt;ParamType&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/load_store/struct.BincodeLoadStore.html\" title=\"struct atomic_store::load_store::BincodeLoadStore\">BincodeLoadStore</a>&lt;ParamType&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ParamType: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::load_store::BincodeLoadStore"]},{"text":"impl&lt;ParamType&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/load_store/struct.ArkLoadStore.html\" title=\"struct atomic_store::load_store::ArkLoadStore\">ArkLoadStore</a>&lt;ParamType&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ParamType: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::load_store::ArkLoadStore"]},{"text":"impl&lt;ResourceAdaptor&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/rolling_log/struct.RollingLog.html\" title=\"struct atomic_store::rolling_log::RollingLog\">RollingLog</a>&lt;ResourceAdaptor&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ResourceAdaptor: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::rolling_log::RollingLog"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/storage_location/struct.StorageLocation.html\" title=\"struct atomic_store::storage_location::StorageLocation\">StorageLocation</a>","synthetic":true,"types":["atomic_store::storage_location::StorageLocation"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"atomic_store/version_sync/struct.VersionSyncHandle.html\" title=\"struct atomic_store::version_sync::VersionSyncHandle\">VersionSyncHandle</a>","synthetic":true,"types":["atomic_store::version_sync::VersionSyncHandle"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()