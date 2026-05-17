<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><meta name="generator" content="rustdoc"><meta name="description" content="axum is an HTTP routing and request-handling library that focuses on ergonomics and modularity."><title>axum - Rust</title><script>if(window.location.protocol!=="file:")document.head.insertAdjacentHTML("beforeend","SourceSerif4-Regular-6b053e98.ttf.woff2,FiraSans-Italic-81dc35de.woff2,FiraSans-Regular-0fe48ade.woff2,FiraSans-MediumItalic-ccf7e434.woff2,FiraSans-Medium-e1aa3f0a.woff2,SourceCodePro-Regular-8badfe75.ttf.woff2,SourceCodePro-Semibold-aa29a496.ttf.woff2".split(",").map(f=>`<link rel="preload" as="font" type="font/woff2"href="/-/rustdoc.static/${f}">`).join(""))</script><link rel="stylesheet" href="/-/rustdoc.static/normalize-9960930a.css"><link rel="stylesheet" href="/-/static/vendored.css?0-0-0-e50152ed411bb913753b1dfd203f22cb8711f097-2026-05-17" media="all" /><link rel="stylesheet" href="/-/rustdoc.static/rustdoc-17e0aaed.css"><meta name="rustdoc-vars" data-root-path="../" data-static-root-path="/-/rustdoc.static/" data-current-crate="axum" data-themes="" data-resource-suffix="-20260413-1.97.0-nightly-17584a181" data-rustdoc-version="1.97.0-nightly (17584a181 2026-04-13)" data-channel="nightly" data-search-js="search-b5634cc7.js" data-stringdex-js="stringdex-2da4960a.js" data-settings-js="settings-170eb4bf.js" ><script src="/-/rustdoc.static/storage-41dd4d93.js"></script><script defer src="../crates-20260413-1.97.0-nightly-17584a181.js"></script><script defer src="/-/rustdoc.static/main-5013f961.js"></script><noscript><link rel="stylesheet" href="/-/rustdoc.static/noscript-f7c3ffd8.css"></noscript><link rel="alternate icon" type="image/png" href="/-/rustdoc.static/favicon-32x32-eab170b8.png"><link rel="icon" type="image/svg+xml" href="/-/rustdoc.static/favicon-044be391.svg"><link rel="stylesheet" href="/-/static/rustdoc-2025-08-20.css?0-0-0-e50152ed411bb913753b1dfd203f22cb8711f097-2026-05-17" media="all" /><link rel="stylesheet" href="/-/static/font-awesome.css?0-0-0-e50152ed411bb913753b1dfd203f22cb8711f097-2026-05-17" media="all" />

<link rel="search" href="/-/static/opensearch.xml" type="application/opensearchdescription+xml" title="Docs.rs" />

<script type="text/javascript">(function() {
    function applyTheme(theme) {
        if (theme) {
            document.documentElement.dataset.docsRsTheme = theme;
        }
    }

    window.addEventListener("storage", ev => {
        if (ev.key === "rustdoc-theme") {
            applyTheme(ev.newValue);
        }
    });

    // see ./storage-change-detection.html for details
    window.addEventListener("message", ev => {
        if (ev.data && ev.data.storage && ev.data.storage.key === "rustdoc-theme") {
            applyTheme(ev.data.storage.value);
        }
    });

    applyTheme(window.localStorage.getItem("rustdoc-theme"));
})();</script></head><body class="rustdoc-page">
<div class="nav-container">
    <div class="container">
        <div class="pure-menu pure-menu-horizontal" role="navigation" aria-label="Main navigation">
            <form action="/releases/search"
                  method="GET"
                  id="nav-search-form"
                  class="landing-search-form-nav  ">

                
                <a href="/" class="pure-menu-heading pure-menu-link docsrs-logo" aria-label="Docs.rs">
                    <span title="Docs.rs"><span class="fa fa-solid fa-cubes " aria-hidden="true"></span></span>
                    <span class="title">Docs.rs</span>
                </a><ul class="pure-menu-list">
    <script id="crate-metadata" type="application/json">
        
        {
            "name": "axum",
            "version": "0.8.9"
        }
    </script><li class="pure-menu-item pure-menu-has-children crate-dropdown">
            <a href="#" class="pure-menu-link crate-name" title="HTTP routing and request handling library that focuses on ergonomics and modularity">
                <span class="fa fa-solid fa-cube " aria-hidden="true"></span>
                <span class="title">axum-0.8.9</span>
            </a><div class="pure-menu-children package-details-menu">
                
                <ul class="pure-menu-list menu-item-divided">
                    <li class="pure-menu-heading" id="crate-title">
                        axum 0.8.9
                        <span id="clipboard" class="svg-clipboard" title="Copy crate name and version information"></span>
                    </li><li class="pure-menu-item">
                        <a href="/axum/0.8.9/axum/" class="pure-menu-link description" id="permalink" title="Get a link to this specific version"><span class="fa fa-solid fa-link " aria-hidden="true"></span> Permalink
                        </a>
                    </li><li class="pure-menu-item">
                        <a href="/crate/axum/latest" class="pure-menu-link description" title="See axum in docs.rs">
                            <span class="fa fa-solid fa-cube " aria-hidden="true"></span> Docs.rs crate page
                        </a>
                    </li><li class="pure-menu-item">
                            <span class="pure-menu-link description license"><span class="fa fa-solid fa-scale-unbalanced-flip " aria-hidden="true"></span>
                            <a href="https://spdx.org/licenses/MIT" class="pure-menu-sublink">MIT</a></span>
                        </li></ul>

                <div class="pure-g menu-item-divided">
                    <div class="pure-u-1-2 right-border">
                        <ul class="pure-menu-list">
                            <li class="pure-menu-heading">Links</li>

                            <li class="pure-menu-item">
                                    <a href="https://github.com/tokio-rs/axum" class="pure-menu-link">
                                        <span class="fa fa-solid fa-house " aria-hidden="true"></span> Homepage
                                    </a>
                                </li><li class="pure-menu-item">
                                    <a href="https://github.com/tokio-rs/axum" class="pure-menu-link">
                                        <span class="fa fa-solid fa-code-branch " aria-hidden="true"></span> Repository
                                    </a>
                                </li><li class="pure-menu-item">
                                <a href="https://crates.io/crates/axum" class="pure-menu-link" title="See axum in crates.io">
                                    <span class="fa fa-solid fa-cube " aria-hidden="true"></span> crates.io
                                </a>
                            </li>

                            
                            <li class="pure-menu-item">
                                <a href="/crate/axum/latest/source/" title="Browse source of axum-0.8.9" class="pure-menu-link">
                                    <span class="fa fa-solid fa-folder-open " aria-hidden="true"></span> Source
                                </a>
                            </li>
                        </ul>
                    </div><div class="pure-u-1-2">
                        <ul class="pure-menu-list" id="topbar-owners">
                            <li class="pure-menu-heading">Owners</li><li class="pure-menu-item">
                                    <a href="https://crates.io/users/carllerche" class="pure-menu-link">
                                        <span class="fa fa-solid fa-user " aria-hidden="true"></span> carllerche
                                    </a>
                                </li><li class="pure-menu-item">
                                    <a href="https://crates.io/users/davidpdrsn" class="pure-menu-link">
                                        <span class="fa fa-solid fa-user " aria-hidden="true"></span> davidpdrsn
                                    </a>
                                </li><li class="pure-menu-item">
                                    <a href="https://crates.io/teams/github:tokio-rs:core" class="pure-menu-link">
                                        <span class="fa fa-solid fa-user " aria-hidden="true"></span> github:tokio-rs:core
                                    </a>
                                </li><li class="pure-menu-item">
                                    <a href="https://crates.io/teams/github:tokio-rs:axum-release" class="pure-menu-link">
                                        <span class="fa fa-solid fa-user " aria-hidden="true"></span> github:tokio-rs:axum-release
                                    </a>
                                </li></ul>
                    </div>
                </div>

                <div class="pure-g menu-item-divided">
                    <div class="pure-u-1-2 right-border">
                        <ul class="pure-menu-list">
                            <li class="pure-menu-heading">Dependencies</li>

                            
                            <li class="pure-menu-item">
                                <div class="pure-menu pure-menu-scrollable sub-menu" tabindex="-1">
                                    <ul class="pure-menu-list">
                                        <li class="pure-menu-item"><a href="/axum-core/^0.5.5/" class="pure-menu-link">
                axum-core ^0.5.5
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/axum-macros/^0.5.1/" class="pure-menu-link">
                axum-macros ^0.5.1
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/base64/^0.22.1/" class="pure-menu-link">
                base64 ^0.22.1
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/bytes/^1.0/" class="pure-menu-link">
                bytes ^1.0
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/form_urlencoded/^1.1.0/" class="pure-menu-link">
                form_urlencoded ^1.1.0
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/futures-util/^0.3/" class="pure-menu-link">
                futures-util ^0.3
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/http/^1.0.0/" class="pure-menu-link">
                http ^1.0.0
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/http-body/^1.0.0/" class="pure-menu-link">
                http-body ^1.0.0
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/http-body-util/^0.1.0/" class="pure-menu-link">
                http-body-util ^0.1.0
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/hyper/^1.1.0/" class="pure-menu-link">
                hyper ^1.1.0
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/hyper-util/^0.1.3/" class="pure-menu-link">
                hyper-util ^0.1.3
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/itoa/^1.0.5/" class="pure-menu-link">
                itoa ^1.0.5
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/matchit/=0.8.4/" class="pure-menu-link">
                matchit =0.8.4
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/memchr/^2.4.1/" class="pure-menu-link">
                memchr ^2.4.1
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/mime/^0.3.16/" class="pure-menu-link">
                mime ^0.3.16
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/multer/^3.0.0/" class="pure-menu-link">
                multer ^3.0.0
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/percent-encoding/^2.1/" class="pure-menu-link">
                percent-encoding ^2.1
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/pin-project-lite/^0.2.7/" class="pure-menu-link">
                pin-project-lite ^0.2.7
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/reqwest/^0.12/" class="pure-menu-link">
                reqwest ^0.12
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/serde/^1.0.211/" class="pure-menu-link">
                serde ^1.0.211
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/serde_core/^1.0.221/" class="pure-menu-link">
                serde_core ^1.0.221
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/serde_json/^1.0/" class="pure-menu-link">
                serde_json ^1.0
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/serde_path_to_error/^0.1.8/" class="pure-menu-link">
                serde_path_to_error ^0.1.8
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/serde_urlencoded/^0.7/" class="pure-menu-link">
                serde_urlencoded ^0.7
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/sha1/^0.10/" class="pure-menu-link">
                sha1 ^0.10
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/sync_wrapper/^1.0.0/" class="pure-menu-link">
                sync_wrapper ^1.0.0
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tokio/^1.44/" class="pure-menu-link">
                tokio ^1.44
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tokio-tungstenite/^0.29.0/" class="pure-menu-link">
                tokio-tungstenite ^0.29.0
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tower/^0.5.2/" class="pure-menu-link">
                tower ^0.5.2
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tower-http/^0.6.8/" class="pure-menu-link">
                tower-http ^0.6.8
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tower-layer/^0.3.2/" class="pure-menu-link">
                tower-layer ^0.3.2
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tower-service/^0.3/" class="pure-menu-link">
                tower-service ^0.3
                
                    <i class="dependencies normal">normal</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tracing/^0.1/" class="pure-menu-link">
                tracing ^0.1
                
                    <i class="dependencies normal">normal</i>
                    
                        <i>optional</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/anyhow/^1.0/" class="pure-menu-link">
                anyhow ^1.0
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/hyper/^1.1.0/" class="pure-menu-link">
                hyper ^1.1.0
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/quickcheck/^1.0/" class="pure-menu-link">
                quickcheck ^1.0
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/quickcheck_macros/^1.0/" class="pure-menu-link">
                quickcheck_macros ^1.0
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/reqwest/^0.12/" class="pure-menu-link">
                reqwest ^0.12
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/serde/^1.0.221/" class="pure-menu-link">
                serde ^1.0.221
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/serde_json/^1.0/" class="pure-menu-link">
                serde_json ^1.0
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/time/^0.3/" class="pure-menu-link">
                time ^0.3
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tokio/^1.44.2/" class="pure-menu-link">
                tokio ^1.44.2
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tokio-stream/^0.1/" class="pure-menu-link">
                tokio-stream ^0.1
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tokio-tungstenite/^0.29.0/" class="pure-menu-link">
                tokio-tungstenite ^0.29.0
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tower/^0.5.2/" class="pure-menu-link">
                tower ^0.5.2
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tower-http/^0.6.8/" class="pure-menu-link">
                tower-http ^0.6.8
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tracing/^0.1/" class="pure-menu-link">
                tracing ^0.1
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/tracing-subscriber/^0.3/" class="pure-menu-link">
                tracing-subscriber ^0.3
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li><li class="pure-menu-item"><a href="/uuid/^1.0/" class="pure-menu-link">
                uuid ^1.0
                
                    <i class="dependencies dev">dev</i>
                    
                
            </a>
        </li>
                                    </ul>
                                </div>
                            </li>
                        </ul>
                    </div>

                    <div class="pure-u-1-2">
                        <ul class="pure-menu-list">
                            <li class="pure-menu-heading">Versions</li>

                            <li class="pure-menu-item">
                                <div class="pure-menu pure-menu-scrollable sub-menu" id="releases-list" tabindex="-1" data-url="/crate/axum/latest/menus/releases/axum/">
                                    <span class="rotate"><span class="fa fa-solid fa-spinner " aria-hidden="true"></span></span>
                                </div>
                            </li>
                        </ul>
                    </div>
                </div>
                    
                    
                    <div class="pure-g">
                        <div class="pure-u-1">
                            <ul class="pure-menu-list">
                                <li>
                                    <a href="/crate/axum/latest" class="pure-menu-link">
                                        <b>100%</b>
                                        of the crate is documented
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </div></div>
        </li><li class="pure-menu-item pure-menu-has-children">
                <a href="#" class="pure-menu-link" aria-label="Platform">
                    <span class="fa fa-solid fa-gears " aria-hidden="true"></span>
                    <span class="title">Platform</span>
                </a>

                
                <ul class="pure-menu-children" id="platforms" data-url="/crate/axum/latest/menus/platforms/axum/"><li class="pure-menu-item">
            <a href="/crate/axum/latest/target-redirect/aarch64-apple-darwin/axum/" class="pure-menu-link" data-fragment="retain" rel="nofollow">aarch64-apple-darwin</a>
        </li><li class="pure-menu-item">
            <a href="/crate/axum/latest/target-redirect/aarch64-unknown-linux-gnu/axum/" class="pure-menu-link" data-fragment="retain" rel="nofollow">aarch64-unknown-linux-gnu</a>
        </li><li class="pure-menu-item">
            <a href="/crate/axum/latest/target-redirect/i686-pc-windows-msvc/axum/" class="pure-menu-link" data-fragment="retain" rel="nofollow">i686-pc-windows-msvc</a>
        </li><li class="pure-menu-item">
            <a href="/crate/axum/latest/target-redirect/x86_64-pc-windows-msvc/axum/" class="pure-menu-link" data-fragment="retain" rel="nofollow">x86_64-pc-windows-msvc</a>
        </li><li class="pure-menu-item">
            <a href="/crate/axum/latest/target-redirect/axum/" class="pure-menu-link" data-fragment="retain" rel="nofollow">x86_64-unknown-linux-gnu</a>
        </li></ul>
            </li><li class="pure-menu-item">
                <a href="/crate/axum/latest/features" title="Browse available feature flags of axum-0.8.9" class="pure-menu-link">
                    <span class="fa fa-solid fa-flag " aria-hidden="true"></span>
                    <span class="title">Feature flags</span>
                </a>
            </li>
        
    
</ul><div class="spacer"></div>
                
                

<ul class="pure-menu-list">
                    <li class="pure-menu-item pure-menu-has-children">
                        <a href="#" class="pure-menu-link" aria-label="docs.rs">docs.rs</a>
                        <ul class="pure-menu-children aligned-icons"><li class="pure-menu-item"><a class="pure-menu-link" href="/about"><span class="fa fa-solid fa-circle-info " aria-hidden="true"></span> About docs.rs</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/badges"><span class="fa fa-brands fa-fonticons " aria-hidden="true"></span> Badges</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/builds"><span class="fa fa-solid fa-gears " aria-hidden="true"></span> Builds</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/metadata"><span class="fa fa-solid fa-table " aria-hidden="true"></span> Metadata</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/redirections"><span class="fa fa-solid fa-road " aria-hidden="true"></span> Shorthand URLs</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/download"><span class="fa fa-solid fa-download " aria-hidden="true"></span> Download</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/about/rustdoc-json"><span class="fa fa-solid fa-file-code " aria-hidden="true"></span> Rustdoc JSON</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="/releases/queue"><span class="fa fa-solid fa-gears " aria-hidden="true"></span> Build queue</a></li><li class="pure-menu-item"><a class="pure-menu-link" href="https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs" target="_blank"><span class="fa fa-solid fa-shield-halved " aria-hidden="true"></span> Privacy policy</a></li>
                        </ul>
                    </li>
                </ul>
                <ul class="pure-menu-list"><li class="pure-menu-item pure-menu-has-children">
                        <a href="#" class="pure-menu-link" aria-label="Rust">Rust</a>
                        <ul class="pure-menu-children">
                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://www.rust-lang.org/" target="_blank">Rust website</a></li>
                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://doc.rust-lang.org/book/" target="_blank">The Book</a></li>

                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://doc.rust-lang.org/std/" target="_blank">Standard Library API Reference</a></li>

                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://doc.rust-lang.org/rust-by-example/" target="_blank">Rust by Example</a></li>

                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://doc.rust-lang.org/cargo/guide/" target="_blank">The Cargo Guide</a></li>

                            <li class="pure-menu-item"><a class="pure-menu-link" href="https://doc.rust-lang.org/nightly/clippy" target="_blank">Clippy Documentation</a></li>
                        </ul>
                    </li>
                </ul>
                
                <div id="search-input-nav">
                    <label for="nav-search">
                        <span class="fa fa-solid fa-magnifying-glass " aria-hidden="true"></span>
                    </label>

                    
                    
                    <input id="nav-search" name="query" type="text" aria-label="Find crate by search query" tabindex="-1"
                        placeholder="Find crate"
                        >
                </div>
            </form>
        </div>
    </div>
</div><div class="rustdoc mod crate container-rustdoc" id="rustdoc_body_wrapper" tabindex="-1"><script async src="/-/static/menu.js?0-0-0-e50152ed411bb913753b1dfd203f22cb8711f097-2026-05-17"></script>
<script async src="/-/static/index.js?0-0-0-e50152ed411bb913753b1dfd203f22cb8711f097-2026-05-17"></script>

<iframe src="/-/storage-change-detection.html" width="0" height="0" style="display: none"></iframe><a class="skip-main-content" href="#main-content">Skip to main content</a><!--[if lte IE 11]><div class="warning">This old browser is unsupported and will most likely display funky things.</div><![endif]--><rustdoc-topbar><h2><a href="#">Crate axum</a></h2></rustdoc-topbar><nav class="sidebar"><div class="sidebar-crate"><h2><a href="../axum/index.html">axum</a><span class="version">0.8.9</span></h2></div><div class="sidebar-elems"><ul class="block"><li><a id="all-types" href="all.html">All Items</a></li></ul><section id="rustdoc-toc"><h3><a href="#">Sections</a></h3><ul class="block top-toc"><li><a href="#high-level-features" title="High-level features">High-level features</a></li><li><a href="#compatibility" title="Compatibility">Compatibility</a></li><li><a href="#example" title="Example">Example</a></li><li><a href="#routing" title="Routing">Routing</a></li><li><a href="#handlers" title="Handlers">Handlers</a></li><li><a href="#extractors" title="Extractors">Extractors</a></li><li><a href="#responses" title="Responses">Responses</a></li><li><a href="#error-handling" title="Error handling">Error handling</a></li><li><a href="#middleware" title="Middleware">Middleware</a></li><li><a href="#sharing-state-with-handlers" title="Sharing state with handlers">Sharing state with handlers</a><ul><li><a href="#using-the-state-extractor" title="Using the `State` extractor">Using the <code>State</code> extractor</a></li><li><a href="#using-request-extensions" title="Using request extensions">Using request extensions</a></li><li><a href="#using-closure-captures" title="Using closure captures">Using closure captures</a></li><li><a href="#using-task-local-variables" title="Using task-local variables">Using task-local variables</a></li></ul></li><li><a href="#building-integrations-for-axum" title="Building integrations for axum">Building integrations for axum</a></li><li><a href="#required-dependencies" title="Required dependencies">Required dependencies</a></li><li><a href="#examples" title="Examples">Examples</a></li><li><a href="#feature-flags" title="Feature flags">Feature flags</a></li></ul><h3><a href="#reexports">Crate Items</a></h3><ul class="block"><li><a href="#reexports" title="Re-exports">Re-exports</a></li><li><a href="#modules" title="Modules">Modules</a></li><li><a href="#structs" title="Structs">Structs</a></li><li><a href="#traits" title="Traits">Traits</a></li><li><a href="#functions" title="Functions">Functions</a></li><li><a href="#types" title="Type Aliases">Type Aliases</a></li><li><a href="#attributes" title="Attribute Macros">Attribute Macros</a></li></ul></section><div id="rustdoc-modnav"></div></div></nav><div class="sidebar-resizer" title="Drag to resize sidebar"></div><main><div class="width-limiter"><section id="main-content" class="content" tabindex="-1"><div class="main-heading"><h1>Crate <span>axum</span>&nbsp;<button id="copy-path" title="Copy item path to clipboard">Copy item path</button></h1><rustdoc-toolbar></rustdoc-toolbar><span class="sub-heading"><a class="src" href="../src/axum/lib.rs.html#1-535">Source</a> </span></div><details class="toggle top-doc" open><summary class="hideme"><span>Expand description</span></summary><div class="docblock"><p>axum is an HTTP routing and request-handling library that focuses on ergonomics and modularity.</p>
<h2 id="high-level-features"><a class="doc-anchor" href="#high-level-features">§</a>High-level features</h2>
<ul>
<li>Route requests to handlers with a macro-free API.</li>
<li>Declaratively parse requests using extractors.</li>
<li>Simple and predictable error handling model.</li>
<li>Generate responses with minimal boilerplate.</li>
<li>Take full advantage of the <a href="https://crates.io/crates/tower"><code>tower</code></a> and <a href="https://crates.io/crates/tower-http"><code>tower-http</code></a> ecosystem of
middleware, services, and utilities.</li>
</ul>
<p>In particular, the last point is what sets <code>axum</code> apart from other libraries / frameworks.
<code>axum</code> doesn’t have its own middleware system but instead uses
<a href="https://docs.rs/tower-service/0.3.3/x86_64-unknown-linux-gnu/tower_service/trait.Service.html" title="trait tower_service::Service"><code>tower::Service</code></a>. This means <code>axum</code> gets timeouts, tracing, compression,
authorization, and more, for free. It also enables you to share middleware with
applications written using <a href="http://crates.io/crates/hyper"><code>hyper</code></a> or <a href="http://crates.io/crates/tonic"><code>tonic</code></a>.</p>
<h2 id="compatibility"><a class="doc-anchor" href="#compatibility">§</a>Compatibility</h2>
<p>axum is designed to work with <a href="https://docs.rs/tokio/1.47.1/x86_64-unknown-linux-gnu/tokio/index.html" title="mod tokio">tokio</a> and <a href="https://docs.rs/hyper/1.7.0/x86_64-unknown-linux-gnu/hyper/index.html" title="mod hyper">hyper</a>. Runtime and
transport layer independence is not a goal, at least for the time being.</p>
<h2 id="example"><a class="doc-anchor" href="#example">§</a>Example</h2>
<p>The “Hello, World!” of axum is:</p>

<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">use </span>axum::{
    routing::get,
    Router,
};

<span class="attr">#[tokio::main]
</span><span class="kw">async fn </span>main() {
    <span class="comment">// build our application with a single route
    </span><span class="kw">let </span>app = Router::new().route(<span class="string">"/"</span>, get(|| <span class="kw">async </span>{ <span class="string">"Hello, World!" </span>}));

    <span class="comment">// run our app with hyper, listening globally on port 3000
    </span><span class="kw">let </span>listener = tokio::net::TcpListener::bind(<span class="string">"0.0.0.0:3000"</span>).<span class="kw">await</span>.unwrap();
    axum::serve(listener, app).<span class="kw">await</span>.unwrap();
}</code></pre></div>
<p>Note using <code>#[tokio::main]</code> requires you enable tokio’s <code>macros</code> and <code>rt-multi-thread</code> features
or just <code>full</code> to enable all features (<code>cargo add tokio --features macros,rt-multi-thread</code>).</p>
<h2 id="routing"><a class="doc-anchor" href="#routing">§</a>Routing</h2>
<p><a href="struct.Router.html" title="struct axum::Router"><code>Router</code></a> is used to set up which paths go to which services:</p>

<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">use </span>axum::{Router, routing::get};

<span class="comment">// our router
</span><span class="kw">let </span>app = Router::new()
    .route(<span class="string">"/"</span>, get(root))
    .route(<span class="string">"/foo"</span>, get(get_foo).post(post_foo))
    .route(<span class="string">"/foo/bar"</span>, get(foo_bar));

<span class="comment">// which calls one of these handlers
</span><span class="kw">async fn </span>root() {}
<span class="kw">async fn </span>get_foo() {}
<span class="kw">async fn </span>post_foo() {}
<span class="kw">async fn </span>foo_bar() {}</code></pre></div>
<p>See <a href="struct.Router.html" title="struct axum::Router"><code>Router</code></a> for more details on routing.</p>
<h2 id="handlers"><a class="doc-anchor" href="#handlers">§</a>Handlers</h2>
<p>In axum a “handler” is an async function that accepts zero or more
<a href="extract/index.html" title="mod axum::extract">“extractors”</a> as arguments and returns something that
can be converted <a href="response/index.html" title="mod axum::response">into a response</a>.</p>
<p>Handlers are where your application logic lives and axum applications are built
by routing between handlers.</p>
<p>See <a href="handler/index.html" title="mod axum::handler"><code>handler</code></a> for more details on handlers.</p>
<h2 id="extractors"><a class="doc-anchor" href="#extractors">§</a>Extractors</h2>
<p>An extractor is a type that implements <a href="extract/trait.FromRequest.html" title="trait axum::extract::FromRequest"><code>FromRequest</code></a> or <a href="extract/trait.FromRequestParts.html" title="trait axum::extract::FromRequestParts"><code>FromRequestParts</code></a>. Extractors are
how you pick apart the incoming request to get the parts your handler needs.</p>

<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">use </span>axum::extract::{Path, Query, Json};
<span class="kw">use </span>std::collections::HashMap;

<span class="comment">// `Path` gives you the path parameters and deserializes them.
</span><span class="kw">async fn </span>path(Path(user_id): Path&lt;u32&gt;) {}

<span class="comment">// `Query` gives you the query parameters and deserializes them.
</span><span class="kw">async fn </span>query(Query(params): Query&lt;HashMap&lt;String, String&gt;&gt;) {}

<span class="comment">// Buffer the request body and deserialize it as JSON into a
// `serde_json::Value`. `Json` supports any type that implements
// `serde::Deserialize`.
</span><span class="kw">async fn </span>json(Json(payload): Json&lt;serde_json::Value&gt;) {}</code></pre></div>
<p>See <a href="extract/index.html" title="mod axum::extract"><code>extract</code></a> for more details on extractors.</p>
<h2 id="responses"><a class="doc-anchor" href="#responses">§</a>Responses</h2>
<p>Anything that implements <a href="response/trait.IntoResponse.html" title="trait axum::response::IntoResponse"><code>IntoResponse</code></a> can be returned from handlers.</p>

<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">use </span>axum::{
    body::Body,
    routing::get,
    response::Json,
    Router,
};
<span class="kw">use </span>serde_json::{Value, json};

<span class="comment">// `&amp;'static str` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
</span><span class="kw">async fn </span>plain_text() -&gt; <span class="kw-2">&amp;</span><span class="lifetime">'static </span>str {
    <span class="string">"foo"
</span>}

<span class="comment">// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
</span><span class="kw">async fn </span>json() -&gt; Json&lt;Value&gt; {
    Json(<span class="macro">json!</span>({ <span class="string">"data"</span>: <span class="number">42 </span>}))
}

<span class="kw">let </span>app = Router::new()
    .route(<span class="string">"/plain_text"</span>, get(plain_text))
    .route(<span class="string">"/json"</span>, get(json));</code></pre></div>
<p>See <a href="response/index.html" title="mod axum::response"><code>response</code></a> for more details on building responses.</p>
<h2 id="error-handling"><a class="doc-anchor" href="#error-handling">§</a>Error handling</h2>
<p>axum aims to have a simple and predictable error handling model. That means
it is simple to convert errors into responses and you are guaranteed that
all errors are handled.</p>
<p>See <a href="error_handling/index.html" title="mod axum::error_handling"><code>error_handling</code></a> for more details on axum’s
error handling model and how to handle errors gracefully.</p>
<h2 id="middleware"><a class="doc-anchor" href="#middleware">§</a>Middleware</h2>
<p>There are several different ways to write middleware for axum. See
<a href="middleware/index.html" title="mod axum::middleware"><code>middleware</code></a> for more details.</p>
<h2 id="sharing-state-with-handlers"><a class="doc-anchor" href="#sharing-state-with-handlers">§</a>Sharing state with handlers</h2>
<p>It is common to share some state between handlers. For example, a
pool of database connections or clients to other services may need to
be shared.</p>
<p>The four most common ways of doing that are:</p>
<ul>
<li>Using the <a href="extract/struct.State.html" title="struct axum::extract::State"><code>State</code></a> extractor</li>
<li>Using request extensions</li>
<li>Using closure captures</li>
<li>Using task-local variables</li>
</ul>
<h3 id="using-the-state-extractor"><a class="doc-anchor" href="#using-the-state-extractor">§</a>Using the <a href="extract/struct.State.html" title="struct axum::extract::State"><code>State</code></a> extractor</h3>
<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">use </span>axum::{
    extract::State,
    routing::get,
    Router,
};
<span class="kw">use </span>std::sync::Arc;

<span class="kw">struct </span>AppState {
    <span class="comment">// ...
</span>}

<span class="kw">let </span>shared_state = Arc::new(AppState { <span class="comment">/* ... */ </span>});

<span class="kw">let </span>app = Router::new()
    .route(<span class="string">"/"</span>, get(handler))
    .with_state(shared_state);

<span class="kw">async fn </span>handler(
    State(state): State&lt;Arc&lt;AppState&gt;&gt;,
) {
    <span class="comment">// ...
</span>}</code></pre></div>
<p>State is cloned for every request. Wrapping your state in <code>Arc</code> makes those
clones cheap. If all fields are already cheap to clone (for example, each field
is itself an <code>Arc</code> or a copy type), you can <code>#[derive(Clone)]</code> directly on the
struct instead.</p>
<h4 id="substates-with-fromref"><a class="doc-anchor" href="#substates-with-fromref">§</a>Substates with <code>FromRef</code></h4>
<p>When a handler only needs part of the application state, use <a href="extract/trait.FromRef.html" title="trait axum::extract::FromRef"><code>FromRef</code></a> to extract
a substate. Implement the trait manually, or derive it with <code>#[derive(FromRef)]</code>
(requires the <code>macros</code> feature):</p>

<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">use </span>axum::{Router, routing::get, extract::{State, FromRef}};

<span class="attr">#[derive(Clone)]
</span><span class="kw">struct </span>AppState {
    api_state: ApiState,
}

<span class="attr">#[derive(Clone)]
</span><span class="kw">struct </span>ApiState {}

<span class="comment">// Teach axum how to produce an `ApiState` from a reference to `AppState`.
</span><span class="kw">impl </span>FromRef&lt;AppState&gt; <span class="kw">for </span>ApiState {
    <span class="kw">fn </span>from_ref(app_state: <span class="kw-2">&amp;</span>AppState) -&gt; ApiState {
        app_state.api_state.clone()
    }
}

<span class="kw">let </span>app = Router::new()
    .route(<span class="string">"/"</span>, get(handler))
    .with_state(AppState { api_state: ApiState {} });

<span class="comment">// This handler receives only the `ApiState` slice; it never sees `AppState`.
</span><span class="kw">async fn </span>handler(State(api_state): State&lt;ApiState&gt;) {}</code></pre></div><h4 id="the-routers-type-parameter"><a class="doc-anchor" href="#the-routers-type-parameter">§</a>The <code>Router&lt;S&gt;</code> type parameter</h4>
<p><code>Router&lt;S&gt;</code> when <code>S</code> is not <code>()</code> means a router that is <em>missing</em> a state of type <code>S</code>. Calling
<a href="struct.Router.html#method.with_state" title="method axum::Router::with_state"><code>.with_state(s)</code></a> provides that state and typically produces a
<code>Router&lt;()&gt;</code>, which is the only form that can be passed to <a href="fn.serve.html" title="fn axum::serve"><code>serve()</code></a>. See
<a href="struct.Router.html#method.with_state" title="method axum::Router::with_state"><code>Router::with_state</code></a> for a full explanation.</p>
<p>You should prefer using <a href="extract/struct.State.html" title="struct axum::extract::State"><code>State</code></a> if possible since it’s more type safe. The downside is that
it’s less dynamic than task-local variables and request extensions.</p>
<p>See <a href="extract/struct.State.html" title="struct axum::extract::State"><code>State</code></a> for more details about accessing state.</p>
<h3 id="using-request-extensions"><a class="doc-anchor" href="#using-request-extensions">§</a>Using request extensions</h3>
<p>Another way to share state with handlers is using <a href="struct.Extension.html" title="struct axum::Extension"><code>Extension</code></a> as
layer and extractor:</p>

<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">use </span>axum::{
    extract::Extension,
    routing::get,
    Router,
};
<span class="kw">use </span>std::sync::Arc;

<span class="kw">struct </span>AppState {
    <span class="comment">// ...
</span>}

<span class="kw">let </span>shared_state = Arc::new(AppState { <span class="comment">/* ... */ </span>});

<span class="kw">let </span>app = Router::new()
    .route(<span class="string">"/"</span>, get(handler))
    .layer(Extension(shared_state));

<span class="kw">async fn </span>handler(
    Extension(state): Extension&lt;Arc&lt;AppState&gt;&gt;,
) {
    <span class="comment">// ...
</span>}</code></pre></div>
<p>The downside to this approach is that you’ll get runtime errors
(specifically a <code>500 Internal Server Error</code> response) if you try and extract
an extension that doesn’t exist, perhaps because you forgot to add the
middleware or because you’re extracting the wrong type.</p>
<h3 id="using-closure-captures"><a class="doc-anchor" href="#using-closure-captures">§</a>Using closure captures</h3>
<p>State can also be passed directly to handlers using closure captures:</p>

<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">use </span>axum::{
    Json,
    extract::{Extension, Path},
    routing::{get, post},
    Router,
};
<span class="kw">use </span>std::sync::Arc;
<span class="kw">use </span>serde::Deserialize;

<span class="kw">struct </span>AppState {
    <span class="comment">// ...
</span>}

<span class="kw">let </span>shared_state = Arc::new(AppState { <span class="comment">/* ... */ </span>});

<span class="kw">let </span>app = Router::new()
    .route(
        <span class="string">"/users"</span>,
        post({
            <span class="kw">let </span>shared_state = Arc::clone(<span class="kw-2">&amp;</span>shared_state);
            <span class="kw">move </span>|body| create_user(body, shared_state)
        }),
    )
    .route(
        <span class="string">"/users/{id}"</span>,
        get({
            <span class="kw">let </span>shared_state = Arc::clone(<span class="kw-2">&amp;</span>shared_state);
            <span class="kw">move </span>|path| get_user(path, shared_state)
        }),
    );

<span class="kw">async fn </span>get_user(Path(user_id): Path&lt;String&gt;, state: Arc&lt;AppState&gt;) {
    <span class="comment">// ...
</span>}

<span class="kw">async fn </span>create_user(Json(payload): Json&lt;CreateUserPayload&gt;, state: Arc&lt;AppState&gt;) {
    <span class="comment">// ...
</span>}

<span class="attr">#[derive(Deserialize)]
</span><span class="kw">struct </span>CreateUserPayload {
    <span class="comment">// ...
</span>}</code></pre></div>
<p>The downside to this approach is that it’s the most verbose approach.</p>
<h3 id="using-task-local-variables"><a class="doc-anchor" href="#using-task-local-variables">§</a>Using task-local variables</h3>
<p>This also allows to share state with <code>IntoResponse</code> implementations:</p>

<div class="example-wrap"><pre class="rust rust-example-rendered"><code><span class="kw">use </span>axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::{<span class="self">self</span>, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
<span class="kw">use </span>tokio::task_local;

<span class="attr">#[derive(Clone)]
</span><span class="kw">struct </span>CurrentUser {
    name: String,
}
<span class="macro">task_local!</span> {
    <span class="kw">pub static </span>USER: CurrentUser;
}

<span class="kw">async fn </span>auth(req: Request, next: Next) -&gt; <span class="prelude-ty">Result</span>&lt;Response, StatusCode&gt; {
    <span class="kw">let </span>auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)<span class="question-mark">?</span>;
    <span class="kw">if let </span><span class="prelude-val">Some</span>(current_user) = authorize_current_user(auth_header).<span class="kw">await </span>{
        <span class="comment">// State is setup here in the middleware
        </span><span class="prelude-val">Ok</span>(USER.scope(current_user, next.run(req)).<span class="kw">await</span>)
    } <span class="kw">else </span>{
        <span class="prelude-val">Err</span>(StatusCode::UNAUTHORIZED)
    }
}
<span class="kw">async fn </span>authorize_current_user(auth_token: <span class="kw-2">&amp;</span>str) -&gt; <span class="prelude-ty">Option</span>&lt;CurrentUser&gt; {
    <span class="prelude-val">Some</span>(CurrentUser {
        name: auth_token.to_string(),
    })
}

<span class="kw">struct </span>UserResponse;

<span class="kw">impl </span>IntoResponse <span class="kw">for </span>UserResponse {
    <span class="kw">fn </span>into_response(<span class="self">self</span>) -&gt; Response {
        <span class="comment">// State is accessed here in the IntoResponse implementation
        </span><span class="kw">let </span>current_user = USER.with(|u| u.clone());
        (StatusCode::OK, current_user.name).into_response()
    }
}

<span class="kw">async fn </span>handler() -&gt; UserResponse {
    UserResponse
}

<span class="kw">let </span>app: Router = Router::new()
    .route(<span class="string">"/"</span>, get(handler))
    .route_layer(middleware::from_fn(auth));</code></pre></div>
<p>The main downside to this approach is that it only works when the async executor being used
has the concept of task-local variables. The example above uses
<a href="https://docs.rs/tokio/1/tokio/macro.task_local.html">tokio’s <code>task_local</code> macro</a>.
smol does not yet offer equivalent functionality at the time of writing (see
<a href="https://github.com/smol-rs/async-executor/issues/139">this GitHub issue</a>).</p>
<h2 id="building-integrations-for-axum"><a class="doc-anchor" href="#building-integrations-for-axum">§</a>Building integrations for axum</h2>
<p>Libraries authors that want to provide <a href="extract/trait.FromRequest.html" title="trait axum::extract::FromRequest"><code>FromRequest</code></a>, <a href="extract/trait.FromRequestParts.html" title="trait axum::extract::FromRequestParts"><code>FromRequestParts</code></a>, or
<a href="response/trait.IntoResponse.html" title="trait axum::response::IntoResponse"><code>IntoResponse</code></a> implementations should depend on the <a href="http://crates.io/crates/axum-core"><code>axum-core</code></a> crate, instead of <code>axum</code> if
possible. <a href="http://crates.io/crates/axum-core"><code>axum-core</code></a> contains core types and traits and is less likely to receive breaking
changes.</p>
<h2 id="required-dependencies"><a class="doc-anchor" href="#required-dependencies">§</a>Required dependencies</h2>
<p>To use axum there are a few dependencies you have to pull in as well:</p>
<div class="example-wrap"><pre class="language-toml"><code>[dependencies]
axum = &quot;&lt;latest-version&gt;&quot;
tokio = { version = &quot;&lt;latest-version&gt;&quot;, features = [&quot;full&quot;] }
tower = &quot;&lt;latest-version&gt;&quot;</code></pre></div>
<p>The <code>"full"</code> feature for tokio isn’t necessary but it’s the easiest way to get started.</p>
<p>Tower isn’t strictly necessary either but helpful for testing. See the
testing example in the repo to learn more about testing axum apps.</p>
<h2 id="examples"><a class="doc-anchor" href="#examples">§</a>Examples</h2>
<p>The axum repo contains <a href="https://github.com/tokio-rs/axum/tree/main/examples">a number of examples</a> that show how to put all the
pieces together.</p>
<h2 id="feature-flags"><a class="doc-anchor" href="#feature-flags">§</a>Feature flags</h2>
<p>axum uses a set of <a href="https://doc.rust-lang.org/cargo/reference/features.html#the-features-section">feature flags</a> to reduce the amount of compiled and
optional dependencies.</p>
<p>The following optional features are available:</p>
<div><table><thead><tr><th>Name</th><th>Description</th><th>Default?</th></tr></thead><tbody>
<tr><td><code>http1</code></td><td>Enables hyper’s <code>http1</code> feature</td><td><span role="img" aria-label="Default feature">✔</span></td></tr>
<tr><td><code>http2</code></td><td>Enables hyper’s <code>http2</code> feature</td><td></td></tr>
<tr><td><code>json</code></td><td>Enables the <a href="struct.Json.html" title="struct axum::Json"><code>Json</code></a> type and some similar convenience functionality</td><td><span role="img" aria-label="Default feature">✔</span></td></tr>
<tr><td><code>macros</code></td><td>Enables optional utility macros</td><td></td></tr>
<tr><td><code>matched-path</code></td><td>Enables capturing of every request’s router path and the <a href="extract/struct.MatchedPath.html" title="struct axum::extract::MatchedPath"><code>MatchedPath</code></a> extractor</td><td><span role="img" aria-label="Default feature">✔</span></td></tr>
<tr><td><code>multipart</code></td><td>Enables parsing <code>multipart/form-data</code> requests with <a href="extract/struct.Multipart.html" title="struct axum::extract::Multipart"><code>Multipart</code></a></td><td></td></tr>
<tr><td><code>original-uri</code></td><td>Enables capturing of every request’s original URI and the <a href="extract/struct.OriginalUri.html" title="struct axum::extract::OriginalUri"><code>OriginalUri</code></a> extractor</td><td><span role="img" aria-label="Default feature">✔</span></td></tr>
<tr><td><code>tokio</code></td><td>Enables <code>tokio</code> as a dependency and <code>axum::serve</code>, <code>SSE</code> and <code>extract::connect_info</code> types.</td><td><span role="img" aria-label="Default feature">✔</span></td></tr>
<tr><td><code>tower-log</code></td><td>Enables <code>tower</code>’s <code>log</code> feature</td><td><span role="img" aria-label="Default feature">✔</span></td></tr>
<tr><td><code>tracing</code></td><td>Log rejections from built-in extractors</td><td><span role="img" aria-label="Default feature">✔</span></td></tr>
<tr><td><code>ws</code></td><td>Enables WebSockets support via <a href="extract/ws/index.html" title="mod axum::extract::ws"><code>extract::ws</code></a></td><td></td></tr>
<tr><td><code>form</code></td><td>Enables the <code>Form</code> extractor</td><td><span role="img" aria-label="Default feature">✔</span></td></tr>
<tr><td><code>query</code></td><td>Enables the <code>Query</code> extractor</td><td><span role="img" aria-label="Default feature">✔</span></td></tr>
</tbody></table>
</div></div></details><h2 id="reexports" class="section-header">Re-exports<a href="#reexports" class="anchor">§</a></h2><dl class="item-table reexports"><dt id="reexport.http"><code>pub use <a class="mod" href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/index.html" title="mod http">http</a>;</code></dt></dl><h2 id="modules" class="section-header">Modules<a href="#modules" class="anchor">§</a></h2><dl class="item-table"><dt><a class="mod" href="body/index.html" title="mod axum::body">body</a></dt><dd>HTTP body utilities.</dd><dt><a class="mod" href="error_handling/index.html" title="mod axum::error_handling">error_<wbr>handling</a></dt><dd>Error handling model and utilities</dd><dt><a class="mod" href="extract/index.html" title="mod axum::extract">extract</a></dt><dd>Types and traits for extracting data from requests.</dd><dt><a class="mod" href="handler/index.html" title="mod axum::handler">handler</a></dt><dd>Async functions that can be used to handle requests.</dd><dt><a class="mod" href="middleware/index.html" title="mod axum::middleware">middleware</a></dt><dd>Utilities for writing middleware</dd><dt><a class="mod" href="response/index.html" title="mod axum::response">response</a></dt><dd>Types and traits for generating responses.</dd><dt><a class="mod" href="routing/index.html" title="mod axum::routing">routing</a></dt><dd>Routing between <a href="https://docs.rs/tower-service/0.3.3/x86_64-unknown-linux-gnu/tower_service/trait.Service.html" title="trait tower_service::Service"><code>Service</code></a>s and handlers.</dd><dt><a class="mod" href="serve/index.html" title="mod axum::serve">serve</a><wbr><span class="stab portability" title="Available on crate feature `tokio` and (crate features `http1` or `http2`) only"><code>tokio</code> and (<code>http1</code> or <code>http2</code>)</span></dt><dd>Serve services.</dd><dt><a class="mod" href="test_helpers/index.html" title="mod axum::test_helpers">test_<wbr>helpers</a><wbr><span class="stab portability" title="Available on crate features `__private` only"><code>__private</code></span></dt></dl><h2 id="structs" class="section-header">Structs<a href="#structs" class="anchor">§</a></h2><dl class="item-table"><dt><a class="struct" href="struct.Error.html" title="struct axum::Error">Error</a></dt><dd>Errors that can happen when using axum.</dd><dt><a class="struct" href="struct.Extension.html" title="struct axum::Extension">Extension</a></dt><dd>Extractor and response for extensions.</dd><dt><a class="struct" href="struct.Form.html" title="struct axum::Form">Form</a><wbr><span class="stab portability" title="Available on crate feature `form` only"><code>form</code></span></dt><dd>URL encoded extractor and response.</dd><dt><a class="struct" href="struct.Json.html" title="struct axum::Json">Json</a><wbr><span class="stab portability" title="Available on crate feature `json` only"><code>json</code></span></dt><dd>JSON Extractor / Response.</dd><dt><a class="struct" href="struct.Router.html" title="struct axum::Router">Router</a></dt><dd>The router type for composing handlers and services.</dd></dl><h2 id="traits" class="section-header">Traits<a href="#traits" class="anchor">§</a></h2><dl class="item-table"><dt><a class="trait" href="trait.RequestExt.html" title="trait axum::RequestExt">Request<wbr>Ext</a></dt><dd>Extension trait that adds additional methods to <a href="extract/type.Request.html" title="type axum::extract::Request"><code>Request</code></a>.</dd><dt><a class="trait" href="trait.RequestPartsExt.html" title="trait axum::RequestPartsExt">Request<wbr>Parts<wbr>Ext</a></dt><dd>Extension trait that adds additional methods to <a href="https://docs.rs/http/1.3.1/x86_64-unknown-linux-gnu/http/request/struct.Parts.html" title="struct http::request::Parts"><code>Parts</code></a>.</dd><dt><a class="trait" href="trait.ServiceExt.html" title="trait axum::ServiceExt">Service<wbr>Ext</a></dt><dd>Extension trait that adds additional methods to any <a href="https://docs.rs/tower-service/0.3.3/x86_64-unknown-linux-gnu/tower_service/trait.Service.html" title="trait tower_service::Service"><code>Service</code></a>.</dd></dl><h2 id="functions" class="section-header">Functions<a href="#functions" class="anchor">§</a></h2><dl class="item-table"><dt><a class="fn" href="fn.serve.html" title="fn axum::serve">serve</a><wbr><span class="stab portability" title="Available on crate feature `tokio` and (crate features `http1` or `http2`) only"><code>tokio</code> and (<code>http1</code> or <code>http2</code>)</span></dt><dd>Serve the service with the supplied listener.</dd></dl><h2 id="types" class="section-header">Type Aliases<a href="#types" class="anchor">§</a></h2><dl class="item-table"><dt><a class="type" href="type.BoxError.html" title="type axum::BoxError">BoxError</a></dt><dd>Alias for a type-erased error type.</dd></dl><h2 id="attributes" class="section-header">Attribute Macros<a href="#attributes" class="anchor">§</a></h2><dl class="item-table"><dt><a class="attr" href="attr.debug_handler.html" title="attr axum::debug_handler">debug_<wbr>handler</a><wbr><span class="stab portability" title="Available on crate feature `macros` only"><code>macros</code></span></dt><dd>Generates better error messages when applied to handler functions.</dd><dt><a class="attr" href="attr.debug_middleware.html" title="attr axum::debug_middleware">debug_<wbr>middleware</a><wbr><span class="stab portability" title="Available on crate feature `macros` only"><code>macros</code></span></dt><dd>Generates better error messages when applied to middleware functions.</dd></dl><script type="text/json" id="notable-traits-data">{"&[u8]":"<h3>Notable traits for <code>&amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]</code></h3><pre><code><div class=\"where\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.Read.html\" title=\"trait std::io::Read\">Read</a> for &amp;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>]</div>"}</script></section></div></main></div></body></html>