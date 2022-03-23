(function() {var implementors = {};
implementors["atomic_store"] = [{"text":"impl&lt;ResourceAdaptor&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/append_log/struct.AppendLog.html\" title=\"struct atomic_store::append_log::AppendLog\">AppendLog</a>&lt;ResourceAdaptor&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ResourceAdaptor: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::append_log::AppendLog"]},{"text":"impl&lt;'a, ResourceAdaptor&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/append_log/struct.Iter.html\" title=\"struct atomic_store::append_log::Iter\">Iter</a>&lt;'a, ResourceAdaptor&gt;","synthetic":true,"types":["atomic_store::append_log::Iter"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/atomic_store/struct.AtomicStoreFileContents.html\" title=\"struct atomic_store::atomic_store::AtomicStoreFileContents\">AtomicStoreFileContents</a>","synthetic":true,"types":["atomic_store::atomic_store::AtomicStoreFileContents"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/atomic_store/struct.AtomicStoreLoader.html\" title=\"struct atomic_store::atomic_store::AtomicStoreLoader\">AtomicStoreLoader</a>","synthetic":true,"types":["atomic_store::atomic_store::AtomicStoreLoader"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/atomic_store/struct.AtomicStore.html\" title=\"struct atomic_store::atomic_store::AtomicStore\">AtomicStore</a>","synthetic":true,"types":["atomic_store::atomic_store::AtomicStore"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"enum\" href=\"atomic_store/error/enum.PersistenceError.html\" title=\"enum atomic_store::error::PersistenceError\">PersistenceError</a>","synthetic":true,"types":["atomic_store::error::PersistenceError"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.FailedToResolvePathSnafu.html\" title=\"struct atomic_store::error::FailedToResolvePathSnafu\">FailedToResolvePathSnafu</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::FailedToResolvePathSnafu"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.FailedToFindExpectedResourceSnafu.html\" title=\"struct atomic_store::error::FailedToFindExpectedResourceSnafu\">FailedToFindExpectedResourceSnafu</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::FailedToFindExpectedResourceSnafu"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.InvalidPathToFileSnafu.html\" title=\"struct atomic_store::error::InvalidPathToFileSnafu\">InvalidPathToFileSnafu</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::InvalidPathToFileSnafu"]},{"text":"impl&lt;__T0, __T1&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.InvalidFileContentsSnafu.html\" title=\"struct atomic_store::error::InvalidFileContentsSnafu\">InvalidFileContentsSnafu</a>&lt;__T0, __T1&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;__T1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::InvalidFileContentsSnafu"]},{"text":"impl&lt;__T0, __T1&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.FailedToWriteToFileSnafu.html\" title=\"struct atomic_store::error::FailedToWriteToFileSnafu\">FailedToWriteToFileSnafu</a>&lt;__T0, __T1&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;__T1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::FailedToWriteToFileSnafu"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.DuplicateResourceKeySnafu.html\" title=\"struct atomic_store::error::DuplicateResourceKeySnafu\">DuplicateResourceKeySnafu</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::DuplicateResourceKeySnafu"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.ResourceFormatInconsistentSnafu.html\" title=\"struct atomic_store::error::ResourceFormatInconsistentSnafu\">ResourceFormatInconsistentSnafu</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::ResourceFormatInconsistentSnafu"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.FeatureNotYetImplementedSnafu.html\" title=\"struct atomic_store::error::FeatureNotYetImplementedSnafu\">FeatureNotYetImplementedSnafu</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::FeatureNotYetImplementedSnafu"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.StdIoDirOpsSnafu.html\" title=\"struct atomic_store::error::StdIoDirOpsSnafu\">StdIoDirOpsSnafu</a>","synthetic":true,"types":["atomic_store::error::StdIoDirOpsSnafu"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.StdIoOpenSnafu.html\" title=\"struct atomic_store::error::StdIoOpenSnafu\">StdIoOpenSnafu</a>","synthetic":true,"types":["atomic_store::error::StdIoOpenSnafu"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.StdIoSeekSnafu.html\" title=\"struct atomic_store::error::StdIoSeekSnafu\">StdIoSeekSnafu</a>","synthetic":true,"types":["atomic_store::error::StdIoSeekSnafu"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.StdIoWriteSnafu.html\" title=\"struct atomic_store::error::StdIoWriteSnafu\">StdIoWriteSnafu</a>","synthetic":true,"types":["atomic_store::error::StdIoWriteSnafu"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.StdIoReadSnafu.html\" title=\"struct atomic_store::error::StdIoReadSnafu\">StdIoReadSnafu</a>","synthetic":true,"types":["atomic_store::error::StdIoReadSnafu"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.BincodeSerSnafu.html\" title=\"struct atomic_store::error::BincodeSerSnafu\">BincodeSerSnafu</a>","synthetic":true,"types":["atomic_store::error::BincodeSerSnafu"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.BincodeDeSnafu.html\" title=\"struct atomic_store::error::BincodeDeSnafu\">BincodeDeSnafu</a>","synthetic":true,"types":["atomic_store::error::BincodeDeSnafu"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.ArkSerSnafu.html\" title=\"struct atomic_store::error::ArkSerSnafu\">ArkSerSnafu</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::ArkSerSnafu"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.ArkDeSnafu.html\" title=\"struct atomic_store::error::ArkDeSnafu\">ArkDeSnafu</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::ArkDeSnafu"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.OtherStoreSnafu.html\" title=\"struct atomic_store::error::OtherStoreSnafu\">OtherStoreSnafu</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::OtherStoreSnafu"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.OtherLoadSnafu.html\" title=\"struct atomic_store::error::OtherLoadSnafu\">OtherLoadSnafu</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::OtherLoadSnafu"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.GlobSyntaxSnafu.html\" title=\"struct atomic_store::error::GlobSyntaxSnafu\">GlobSyntaxSnafu</a>","synthetic":true,"types":["atomic_store::error::GlobSyntaxSnafu"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.GlobRuntimeSnafu.html\" title=\"struct atomic_store::error::GlobRuntimeSnafu\">GlobRuntimeSnafu</a>","synthetic":true,"types":["atomic_store::error::GlobRuntimeSnafu"]},{"text":"impl&lt;__T0&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.SyncPoisonSnafu.html\" title=\"struct atomic_store::error::SyncPoisonSnafu\">SyncPoisonSnafu</a>&lt;__T0&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;__T0: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::error::SyncPoisonSnafu"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/error/struct.TimedOutSnafu.html\" title=\"struct atomic_store::error::TimedOutSnafu\">TimedOutSnafu</a>","synthetic":true,"types":["atomic_store::error::TimedOutSnafu"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/fixed_append_log/struct.IndexContents.html\" title=\"struct atomic_store::fixed_append_log::IndexContents\">IndexContents</a>","synthetic":true,"types":["atomic_store::fixed_append_log::IndexContents"]},{"text":"impl&lt;ResourceAdaptor&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/fixed_append_log/struct.FixedAppendLog.html\" title=\"struct atomic_store::fixed_append_log::FixedAppendLog\">FixedAppendLog</a>&lt;ResourceAdaptor&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ResourceAdaptor: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::fixed_append_log::FixedAppendLog"]},{"text":"impl&lt;'a, ResourceAdaptor&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/fixed_append_log/struct.Iter.html\" title=\"struct atomic_store::fixed_append_log::Iter\">Iter</a>&lt;'a, ResourceAdaptor&gt;","synthetic":true,"types":["atomic_store::fixed_append_log::Iter"]},{"text":"impl&lt;ParamType&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/load_store/struct.BincodeLoadStore.html\" title=\"struct atomic_store::load_store::BincodeLoadStore\">BincodeLoadStore</a>&lt;ParamType&gt;","synthetic":true,"types":["atomic_store::load_store::BincodeLoadStore"]},{"text":"impl&lt;ParamType&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/load_store/struct.ArkLoadStore.html\" title=\"struct atomic_store::load_store::ArkLoadStore\">ArkLoadStore</a>&lt;ParamType&gt;","synthetic":true,"types":["atomic_store::load_store::ArkLoadStore"]},{"text":"impl&lt;ResourceAdaptor&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/rolling_log/struct.RollingLog.html\" title=\"struct atomic_store::rolling_log::RollingLog\">RollingLog</a>&lt;ResourceAdaptor&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ResourceAdaptor: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a>,&nbsp;</span>","synthetic":true,"types":["atomic_store::rolling_log::RollingLog"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/storage_location/struct.StorageLocation.html\" title=\"struct atomic_store::storage_location::StorageLocation\">StorageLocation</a>","synthetic":true,"types":["atomic_store::storage_location::StorageLocation"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Freeze.html\" title=\"trait core::marker::Freeze\">Freeze</a> for <a class=\"struct\" href=\"atomic_store/version_sync/struct.VersionSyncHandle.html\" title=\"struct atomic_store::version_sync::VersionSyncHandle\">VersionSyncHandle</a>","synthetic":true,"types":["atomic_store::version_sync::VersionSyncHandle"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()