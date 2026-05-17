<!DOCTYPE html>
<html lang="en">

<head>
	<meta charset="UTF-8" />
	<meta name="awa-expId" content="vscw_aaflight1016_treatment:103440;" />
	<meta name="awa-env" content="prod" />
	<meta name="viewport" content="width=device-width, initial-scale=1.0">
	<meta name="google-site-verification" content="hNs7DXrTySP_X-0P_AC0WulAXvUwgSXEmgfcO2r79dw" />

	<!-- Twitter and Facebook OpenGraph Metadata-->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:site" content="@code" />

	<meta name="keywords" content="vscode API" />

<meta name="description" content="Explain the structure of a Visual Studio Code extension (plug-in)" />

<meta name="ms.prod" content="vs-code" />
<meta name="ms.TOCTitle" content="Extension Anatomy" />
<meta name="ms.ContentId" content="8027f6fb-6c9e-4106-8ef1-f9b0ba1b7085" />
<meta name="ms.topic" content="conceptual" />
<meta name="ms.date" content="5/13/2026" />
<!-- Twitter and Facebook OpenGraph Metadata-->
<meta name="twitter:card" content="summary_large_image" />
<meta property="og:url" content="https://code.visualstudio.com/api/get-started/extension-anatomy" />

<meta property="og:description" content="Explain the structure of a Visual Studio Code extension (plug-in)" />

<meta property="og:type" content="article" />
<meta property="og:title" content="Extension Anatomy" />

<meta property="og:image" content="https://code.visualstudio.com/opengraphimg/generated/api/get-started/extension-anatomy.webp" />



	<link rel="shortcut icon" href="/assets/favicon.ico" sizes="128x128" />
	<link rel="apple-touch-icon" href="/assets/apple-touch-icon.png">

	<title>Extension Anatomy | Visual Studio Code Extension
API</title>

	<link rel="stylesheet" href="/dist/style.css">

	<script src="https://consentdeliveryfd.azurefd.net/mscc/lib/v2/wcp-consent.js"></script>
	<script type="text/javascript" src="https://js.monitor.azure.com/scripts/c/ms.analytics-web-4.min.js"></script>
	
	<script type="text/javascript">
	// Leave as var; siteConsent is initialized and referenced elsewhere.
	var siteConsent = null;
	
	const GPC_DataSharingOptIn = false;
	WcpConsent.onInitCallback(function () {
		window.appInsights = new oneDS.ApplicationInsights();
		window.appInsights.initialize({
			instrumentationKey: "1a3eb3104447440391ad5f2a6ee06a0a-62879566-bc58-4741-9650-302bf2af703f-7103",
			propertyConfiguration: {
				userConsented: false,
				gpcDataSharingOptIn: false,
				callback: {
					userConsentDetails: siteConsent ? siteConsent.getConsent : undefined
				},
			},
			cookieCfg: {
				ignoreCookies: ["MSCC"]
			},
			webAnalyticsConfiguration:{ // Web Analytics Plugin configuration
				urlCollectQuery: true,
				urlCollectHash: true,
				autoCapture: {
					scroll: true,
					pageView: true,
					onLoad: true,
					onUnload: true,
					click: true,
					resize: true,
					jsError: true
				}
			}
		}, []);
	
		window.appInsights.getPropertyManager().getPropertiesContext().web.gpcDataSharingOptIn = GPC_DataSharingOptIn;
	});
	</script>
	<link rel="alternate" type="application/atom+xml" title="RSS Feed for code.visualstudio.com" href="/feed.xml" />
</head>

<body >
	<!-- Setting theme here to avoid FOUC -->
	<script>
		function setTheme(themeName) {
			if (themeName === 'dark') {
				document.documentElement.removeAttribute('data-theme'); // dark is default, so no data-theme attribute needed
			}

			if (themeName === 'light') {
				document.documentElement.setAttribute('data-theme', themeName);
			}
			return;
		}

		// Determine initial theme: user preference or system preference
		let theme = localStorage.getItem('theme') || (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
		setTheme(theme); // Apply the initial theme

		// Listen for changes in the system theme preference
		window.matchMedia('(prefers-color-scheme: dark)').addListener(e => {
			if (!localStorage.getItem('theme')) { // Only if no user preference is saved
				setTheme(e.matches ? 'dark' : 'light');
			}
		});
	</script>

	<div id="main">
		<div class="navbar-fixed-container">
			<div class="navbar navbar-inverse navbar-fixed-top ">
				<div id='cookie-banner'></div>		<nav role="navigation" aria-label="Top Level">
					<div class="container">
						<div class="nav navbar-header">
							<a class="navbar-brand" href="/"><span>Visual Studio Code</span></a>
						</div>
						<div class="navbar-collapse collapse">
							<ul class="nav navbar-nav navbar-left">
								<li class="nav-dropdown">
									<a id="nav-features" href="#" class="nav-dropdown-toggle" role="button" aria-haspopup="true" aria-expanded="false">
										Features <span class="nav-dropdown-caret" aria-hidden="true"></span>
									</a>
									<ul class="nav-dropdown-menu" role="menu">
										<li role="none"><a role="menuitem" href="/features/agents">Agents</a></li>
									</ul>
								</li>
								<li class="nav-dropdown active">
									<a id="nav-docs" href="#" class="nav-dropdown-toggle" role="button" aria-haspopup="true" aria-expanded="false">
										Docs <span class="nav-dropdown-caret" aria-hidden="true"></span>
									</a>
									<ul class="nav-dropdown-menu" role="menu">
										<li role="none"><a role="menuitem" href="/docs">Documentation</a></li>
										<li role="none"><a role="menuitem" href="/api">API</a></li>
									</ul>
								</li>
								<li ><a id="nav-updates" href="/updates">Updates</a>
								</li>
								<li ><a id="nav-blogs" href="/blogs">Blog</a></li>
								<li><a href="https://marketplace.visualstudio.com/VSCode" target="_blank" rel="noopener"
										id="nav-extensions">Extensions</a></li>
								<li ><a id="nav-mcp" href="/mcp">MCP</a></li>
								<li ><a id="nav-faqs" href="/docs/supporting/faq">FAQ</a>
								</li>
								<li ><a id="nav-learn" href="/learn">Learn</a></li>
								<li><a id="nav-events" href="https://aka.ms/vscode/live" target="_blank" rel="noopener">Events</a></li>
							</ul>
							<ul class="nav navbar-nav navbar-right" role="presentation">
								<li>
									<a class="link-button" href="/Download" id="nav-download">
										<span>Download</span>
									</a>
								</li>
							</ul>
						</div>
						<div class="navbar-actions">
							<div class="search" role="presentation">
								<div class="nav-search search-control" role="button" tabindex="0" aria-label="Open search dialog">
									<div class="input-group" role="presentation">
										<span class="input-group-btn">
											<span class="btn search-icon-container" aria-hidden="true">
												<img class="search-icon-dark" src="/assets/icons/search-dark.svg" alt="" />
												<img class="search-icon-light" src="/assets/icons/search.svg" alt="" />
											</span>
										</span>
										<span class="search-box form-control" aria-hidden="true" role="presentation"></span>
										<span class="search-shortcut-placeholder">Search</span>
										<span class="search-shortcut-overlay"></span>
									</div>
								</div>					</div>
							<button type="button" class="theme-switch" id="theme-toggle">
								<img class="theme-icon-light" src="/assets/icons/theme-light.svg" alt="Switch to the dark theme" />
								<img class="theme-icon-dark" src="/assets/icons/theme-dark.svg" alt="Switch to the light theme" />
							</button>
							<a class="link-button navbar-actions-download" href="/Download">
								<span>Download</span>
							</a>
						</div>
						<button type="button" class="navbar-toggle" data-toggle="collapse" data-target=".navbar-collapse"
							aria-label="Expand and Collapse Menu">
							<span class="icon-bar"></span>
							<span class="icon-bar"></span>
							<span class="icon-bar"></span>
						</button>
					</div>
				</nav>
			</div>
		</div>		<div data-announcement-version="2026-05-13-agents-window" class="updates-banner js-hidden  ">
			<div class="container">
				<p class="message"><a href="https://code.visualstudio.com/docs/copilot/agents/agents-window?source=vsc-website-banner" target="_self" rel="noopener">Use the Agents window to build in an agent-first way.</a></p>
			</div>
			<div tabindex="0" role="button" title="Dismiss this update" class="dismiss-btn" id="banner-dismiss-btn"><span class="sr-only">Dismiss this update</span><span aria-hidden="true" class="glyph-icon"></span></div>
		</div>
		<!-- This div wraps around the entire site -->
		<!-- The body itself should already have a main tag -->
		<main id="main-content">
			<div class="body-content docs docs-github-layout">
	<div class="docs-layout-wrapper">
		<!-- Left sidebar - Table of Contents -->
		<aside class="docs-left-sidebar">
			<nav id="docs-navbar" aria-label="Topics" class="docs-nav visible-md visible-lg">
			  <h4>Extension API</h4>
			  <ul class="nav" id="main-nav">
			  <li >
			    <a href="/api" >Overview</a>
			  </li>
			  
			<li class="panel active expanded">
			  <a class="area" role="button" href="#get-started-articles" data-parent="#main-nav" data-toggle="collapse">Get Started</a>
			  <ul id="get-started-articles" class="collapse in">
			
			        <li >
			          <a href="/api/get-started/your-first-extension" >Your First Extension</a>
			        </li>
			          
			        <li class="active">
			          <a href="/api/get-started/extension-anatomy" aria-label="Current Page: Extension Anatomy">Extension Anatomy</a>
			        </li>
			          
			        <li >
			          <a href="/api/get-started/wrapping-up" >Wrapping Up</a>
			        </li>
			          
			  </ul>
			</li>
			    
			<li class="panel collapsed">
			  <a class="area" role="button" href="#extension-capabilities-articles" data-parent="#main-nav" data-toggle="collapse">Extension Capabilities</a>
			  <ul id="extension-capabilities-articles" class="collapse ">
			
			        <li >
			          <a href="/api/extension-capabilities/overview" >Overview</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-capabilities/common-capabilities" >Common Capabilities</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-capabilities/theming" >Theming</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-capabilities/extending-workbench" >Extending Workbench</a>
			        </li>
			          
			  </ul>
			</li>
			    
			<li class="panel collapsed">
			  <a class="area" role="button" href="#extension-guides-articles" data-parent="#main-nav" data-toggle="collapse">Extension Guides</a>
			  <ul id="extension-guides-articles" class="collapse ">
			
			        <li >
			          <a href="/api/extension-guides/overview" >Overview</a>
			        </li>
			          
			<li class="panel collapsed">
			  <a class="area" role="button" href="#extension-guides-ai-articles" data-parent="#extension-guides-articles" data-toggle="collapse">AI</a>
			  <ul id="extension-guides-ai-articles" class="collapse ">
			
			        <li >
			          <a href="/api/extension-guides/ai/ai-extensibility-overview" >AI Extensibility</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/ai/tools" >Language Model Tool</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/ai/mcp" >MCP Dev Guide</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/ai/chat" >Chat Participant</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/ai/chat-tutorial" >Chat Tutorial</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/ai/language-model" >Language Model</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/ai/language-model-tutorial" >Language Model Tutorial</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/ai/language-model-chat-provider" >Language Model Chat Provider</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/ai/prompt-tsx" >Prompt TSX</a>
			        </li>
			          
			  </ul>
			</li>
			        
			        <li >
			          <a href="/api/extension-guides/command" >Command</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/color-theme" >Color Theme</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/file-icon-theme" >File Icon Theme</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/product-icon-theme" >Product Icon Theme</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/tree-view" >Tree View</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/webview" >Webview</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/notebook" >Notebook</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/custom-editors" >Custom Editors</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/virtual-documents" >Virtual Documents</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/virtual-workspaces" >Virtual Workspaces</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/web-extensions" >Web Extensions</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/workspace-trust" >Workspace Trust</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/task-provider" >Task Provider</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/scm-provider" >Source Control</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/debugger-extension" >Debugger Extension</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/markdown-extension" >Markdown Extension</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/testing" >Test Extension</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/custom-data-extension" >Custom Data Extension</a>
			        </li>
			          
			        <li >
			          <a href="/api/extension-guides/telemetry" >Telemetry</a>
			        </li>
			          
			  </ul>
			</li>
			    
			<li class="panel collapsed">
			  <a class="area" role="button" href="#ux-guidelines-articles" data-parent="#main-nav" data-toggle="collapse">UX Guidelines</a>
			  <ul id="ux-guidelines-articles" class="collapse ">
			
			        <li >
			          <a href="/api/ux-guidelines/overview" >Overview</a>
			        </li>
			          
			        <li >
			          <a href="/api/ux-guidelines/activity-bar" >Activity Bar</a>
			        </li>
			          
			        <li >
			          <a href="/api/ux-guidelines/sidebars" >Sidebars</a>
			        </li>
			          
			        <li >
			          <a href="/api/ux-guidelines/panel" >Panel</a>
			        </li>
			          
			        <li >
			          <a href="/api/ux-guidelines/status-bar" >Status Bar</a>
			        </li>
			          
			        <li >
			          <a href="/api/ux-guidelines/views" >Views</a>
			        </li>
			          
			        <li >
			          <a href="/api/ux-guidelines/editor-actions" >Editor Actions</a>
			        </li>
			          
			        <li >
			          <a href="/api/ux-guidelines/quick-picks" >Quick Picks</a>
			        </li>
			          
			        <li >
			          <a href="/api/ux-guidelines/command-palette" >Command Palette</a>
			        </li>
			          
			        <li >
			          <a href="/api/ux-guidelines/notifications" >Notifications</a>
			        </li>
			          
			        <li >
			          <a href="/api/ux-guidelines/webviews" >Webviews</a>
			        </li>
			          
			        <li >
			          <a href="/api/ux-guidelines/context-menus" >Context Menus</a>
			        </li>
			          
			        <li >
			          <a href="/api/ux-guidelines/walkthroughs" >Walkthroughs</a>
			        </li>
			          
			        <li >
			          <a href="/api/ux-guidelines/settings" >Settings</a>
			        </li>
			          
			  </ul>
			</li>
			    
			<li class="panel collapsed">
			  <a class="area" role="button" href="#language-extensions-articles" data-parent="#main-nav" data-toggle="collapse">Language Extensions</a>
			  <ul id="language-extensions-articles" class="collapse ">
			
			        <li >
			          <a href="/api/language-extensions/overview" >Overview</a>
			        </li>
			          
			        <li >
			          <a href="/api/language-extensions/syntax-highlight-guide" >Syntax Highlight Guide</a>
			        </li>
			          
			        <li >
			          <a href="/api/language-extensions/semantic-highlight-guide" >Semantic Highlight Guide</a>
			        </li>
			          
			        <li >
			          <a href="/api/language-extensions/snippet-guide" >Snippet Guide</a>
			        </li>
			          
			        <li >
			          <a href="/api/language-extensions/language-configuration-guide" >Language Configuration Guide</a>
			        </li>
			          
			        <li >
			          <a href="/api/language-extensions/programmatic-language-features" >Programmatic Language Features</a>
			        </li>
			          
			        <li >
			          <a href="/api/language-extensions/language-server-extension-guide" >Language Server Extension Guide</a>
			        </li>
			          
			        <li >
			          <a href="/api/language-extensions/embedded-languages" >Embedded Languages</a>
			        </li>
			          
			  </ul>
			</li>
			    
			<li class="panel collapsed">
			  <a class="area" role="button" href="#working-with-extensions-articles" data-parent="#main-nav" data-toggle="collapse">Testing and Publishing</a>
			  <ul id="working-with-extensions-articles" class="collapse ">
			
			        <li >
			          <a href="/api/working-with-extensions/testing-extension" >Testing Extensions</a>
			        </li>
			          
			        <li >
			          <a href="/api/working-with-extensions/publishing-extension" >Publishing Extensions</a>
			        </li>
			          
			        <li >
			          <a href="/api/working-with-extensions/bundling-extension" >Bundling Extensions</a>
			        </li>
			          
			        <li >
			          <a href="/api/working-with-extensions/continuous-integration" >Continuous Integration</a>
			        </li>
			          
			  </ul>
			</li>
			    
			<li class="panel collapsed">
			  <a class="area" role="button" href="#advanced-topics-articles" data-parent="#main-nav" data-toggle="collapse">Advanced Topics</a>
			  <ul id="advanced-topics-articles" class="collapse ">
			
			        <li >
			          <a href="/api/advanced-topics/extension-host" >Extension Host</a>
			        </li>
			          
			        <li >
			          <a href="/api/advanced-topics/remote-extensions" >Remote Development and Codespaces</a>
			        </li>
			          
			        <li >
			          <a href="/api/advanced-topics/using-proposed-api" >Using Proposed API</a>
			        </li>
			          
			        <li >
			          <a href="/api/advanced-topics/tslint-eslint-migration" >Migrate from TSLint to ESLint</a>
			        </li>
			          
			        <li >
			          <a href="/api/advanced-topics/python-extension-template" >Python Extension Template</a>
			        </li>
			          
			  </ul>
			</li>
			    
			<li class="panel collapsed">
			  <a class="area" role="button" href="#references-articles" data-parent="#main-nav" data-toggle="collapse">References</a>
			  <ul id="references-articles" class="collapse ">
			
			        <li >
			          <a href="/api/references/vscode-api" >VS Code API</a>
			        </li>
			          
			        <li >
			          <a href="/api/references/contribution-points" >Contribution Points</a>
			        </li>
			          
			        <li >
			          <a href="/api/references/activation-events" >Activation Events</a>
			        </li>
			          
			        <li >
			          <a href="/api/references/extension-manifest" >Extension Manifest</a>
			        </li>
			          
			        <li >
			          <a href="/api/references/commands" >Built-In Commands</a>
			        </li>
			          
			        <li >
			          <a href="/api/references/when-clause-contexts" >When Clause Contexts</a>
			        </li>
			          
			        <li >
			          <a href="/api/references/theme-color" >Theme Color</a>
			        </li>
			          
			        <li >
			          <a href="/api/references/icons-in-labels" >Product Icon Reference</a>
			        </li>
			          
			        <li >
			          <a href="/api/references/document-selector" >Document Selector</a>
			        </li>
			          
			  </ul>
			</li>
			    
			  </ul>
			</nav>
			
			<nav id="small-nav" aria-label="Topics" class="docs-nav hidden-md hidden-lg">
				<label class="faux-h4" for="small-nav-dropdown">Topics</label>
				<select id="small-nav-dropdown" aria-label="topics">
			    <option value="/api" >Overview</option>
			
			  <optgroup label="Get Started">
			
			        <option value="/api/get-started/your-first-extension" >Your First Extension</option>
			        
			        <option value="/api/get-started/extension-anatomy" selected>Extension Anatomy</option>
			        
			        <option value="/api/get-started/wrapping-up" >Wrapping Up</option>
			        
			  </optgroup>
			  
			  <optgroup label="Extension Capabilities">
			
			        <option value="/api/extension-capabilities/overview" >Overview</option>
			        
			        <option value="/api/extension-capabilities/common-capabilities" >Common Capabilities</option>
			        
			        <option value="/api/extension-capabilities/theming" >Theming</option>
			        
			        <option value="/api/extension-capabilities/extending-workbench" >Extending Workbench</option>
			        
			  </optgroup>
			  
			  <optgroup label="Extension Guides">
			
			        <option value="/api/extension-guides/overview" >Overview</option>
			        
			        <optgroup label="Extension Guides - AI">
			
			          <option value="/api/extension-guides/ai/ai-extensibility-overview" >AI Extensibility</option>
			          
			          <option value="/api/extension-guides/ai/tools" >Language Model Tool</option>
			          
			          <option value="/api/extension-guides/ai/mcp" >MCP Dev Guide</option>
			          
			          <option value="/api/extension-guides/ai/chat" >Chat Participant</option>
			          
			          <option value="/api/extension-guides/ai/chat-tutorial" >Chat Tutorial</option>
			          
			          <option value="/api/extension-guides/ai/language-model" >Language Model</option>
			          
			          <option value="/api/extension-guides/ai/language-model-tutorial" >Language Model Tutorial</option>
			          
			          <option value="/api/extension-guides/ai/language-model-chat-provider" >Language Model Chat Provider</option>
			          
			          <option value="/api/extension-guides/ai/prompt-tsx" >Prompt TSX</option>
			          
			        </optgroup>
			
			        <option value="/api/extension-guides/command" >Command</option>
			        
			        <option value="/api/extension-guides/color-theme" >Color Theme</option>
			        
			        <option value="/api/extension-guides/file-icon-theme" >File Icon Theme</option>
			        
			        <option value="/api/extension-guides/product-icon-theme" >Product Icon Theme</option>
			        
			        <option value="/api/extension-guides/tree-view" >Tree View</option>
			        
			        <option value="/api/extension-guides/webview" >Webview</option>
			        
			        <option value="/api/extension-guides/notebook" >Notebook</option>
			        
			        <option value="/api/extension-guides/custom-editors" >Custom Editors</option>
			        
			        <option value="/api/extension-guides/virtual-documents" >Virtual Documents</option>
			        
			        <option value="/api/extension-guides/virtual-workspaces" >Virtual Workspaces</option>
			        
			        <option value="/api/extension-guides/web-extensions" >Web Extensions</option>
			        
			        <option value="/api/extension-guides/workspace-trust" >Workspace Trust</option>
			        
			        <option value="/api/extension-guides/task-provider" >Task Provider</option>
			        
			        <option value="/api/extension-guides/scm-provider" >Source Control</option>
			        
			        <option value="/api/extension-guides/debugger-extension" >Debugger Extension</option>
			        
			        <option value="/api/extension-guides/markdown-extension" >Markdown Extension</option>
			        
			        <option value="/api/extension-guides/testing" >Test Extension</option>
			        
			        <option value="/api/extension-guides/custom-data-extension" >Custom Data Extension</option>
			        
			        <option value="/api/extension-guides/telemetry" >Telemetry</option>
			        
			  </optgroup>
			  
			  <optgroup label="UX Guidelines">
			
			        <option value="/api/ux-guidelines/overview" >Overview</option>
			        
			        <option value="/api/ux-guidelines/activity-bar" >Activity Bar</option>
			        
			        <option value="/api/ux-guidelines/sidebars" >Sidebars</option>
			        
			        <option value="/api/ux-guidelines/panel" >Panel</option>
			        
			        <option value="/api/ux-guidelines/status-bar" >Status Bar</option>
			        
			        <option value="/api/ux-guidelines/views" >Views</option>
			        
			        <option value="/api/ux-guidelines/editor-actions" >Editor Actions</option>
			        
			        <option value="/api/ux-guidelines/quick-picks" >Quick Picks</option>
			        
			        <option value="/api/ux-guidelines/command-palette" >Command Palette</option>
			        
			        <option value="/api/ux-guidelines/notifications" >Notifications</option>
			        
			        <option value="/api/ux-guidelines/webviews" >Webviews</option>
			        
			        <option value="/api/ux-guidelines/context-menus" >Context Menus</option>
			        
			        <option value="/api/ux-guidelines/walkthroughs" >Walkthroughs</option>
			        
			        <option value="/api/ux-guidelines/settings" >Settings</option>
			        
			  </optgroup>
			  
			  <optgroup label="Language Extensions">
			
			        <option value="/api/language-extensions/overview" >Overview</option>
			        
			        <option value="/api/language-extensions/syntax-highlight-guide" >Syntax Highlight Guide</option>
			        
			        <option value="/api/language-extensions/semantic-highlight-guide" >Semantic Highlight Guide</option>
			        
			        <option value="/api/language-extensions/snippet-guide" >Snippet Guide</option>
			        
			        <option value="/api/language-extensions/language-configuration-guide" >Language Configuration Guide</option>
			        
			        <option value="/api/language-extensions/programmatic-language-features" >Programmatic Language Features</option>
			        
			        <option value="/api/language-extensions/language-server-extension-guide" >Language Server Extension Guide</option>
			        
			        <option value="/api/language-extensions/embedded-languages" >Embedded Languages</option>
			        
			  </optgroup>
			  
			  <optgroup label="Testing and Publishing">
			
			        <option value="/api/working-with-extensions/testing-extension" >Testing Extensions</option>
			        
			        <option value="/api/working-with-extensions/publishing-extension" >Publishing Extensions</option>
			        
			        <option value="/api/working-with-extensions/bundling-extension" >Bundling Extensions</option>
			        
			        <option value="/api/working-with-extensions/continuous-integration" >Continuous Integration</option>
			        
			  </optgroup>
			  
			  <optgroup label="Advanced Topics">
			
			        <option value="/api/advanced-topics/extension-host" >Extension Host</option>
			        
			        <option value="/api/advanced-topics/remote-extensions" >Remote Development and Codespaces</option>
			        
			        <option value="/api/advanced-topics/using-proposed-api" >Using Proposed API</option>
			        
			        <option value="/api/advanced-topics/tslint-eslint-migration" >Migrate from TSLint to ESLint</option>
			        
			        <option value="/api/advanced-topics/python-extension-template" >Python Extension Template</option>
			        
			  </optgroup>
			  
			  <optgroup label="References">
			
			        <option value="/api/references/vscode-api" >VS Code API</option>
			        
			        <option value="/api/references/contribution-points" >Contribution Points</option>
			        
			        <option value="/api/references/activation-events" >Activation Events</option>
			        
			        <option value="/api/references/extension-manifest" >Extension Manifest</option>
			        
			        <option value="/api/references/commands" >Built-In Commands</option>
			        
			        <option value="/api/references/when-clause-contexts" >When Clause Contexts</option>
			        
			        <option value="/api/references/theme-color" >Theme Color</option>
			        
			        <option value="/api/references/icons-in-labels" >Product Icon Reference</option>
			        
			        <option value="/api/references/document-selector" >Document Selector</option>
			        
			  </optgroup>
			  
				</select>
			</nav>		</aside>
		
		<!-- Content wrapper contains main content + right sidebar -->
		<div class="docs-content-wrapper">
			<!-- Main article content -->
			<main class="docs-main-content body">
				<h1>Extension Anatomy</h1>
<p>In the last topic, you were able to get a basic extension running. How does it work under the hood?</p>
<p>The <code>Hello World</code> extension does 3 things:</p>
<ul>
<li>Registers the <a href="/api/references/activation-events#onCommand"><code>onCommand</code></a> <a href="/api/references/activation-events"><strong>Activation Event</strong></a>: <code>onCommand:helloworld.helloWorld</code>, so the extension becomes activated when user runs the <code>Hello World</code> command.<blockquote><p><strong>Note:</strong> Starting with <a href="https://code.visualstudio.com/updates/v1_74#_implicit-activation-events-for-declared-extension-contributions">VS Code 1.74.0</a>, commands declared in the <code>commands</code> section of <code>package.json</code> automatically activate the extension when invoked, without requiring an explicit <code>onCommand</code> entry in <code>activationEvents</code>.</p>
</blockquote></li>
<li>Uses the <a href="/api/references/contribution-points#contributes.commands"><code>contributes.commands</code></a> <a href="/api/references/contribution-points"><strong>Contribution Point</strong></a> to make the command <code>Hello World</code> available in the Command Palette, and bind it to a command ID <code>helloworld.helloWorld</code>.</li>
<li>Uses the <a href="/api/references/vscode-api#commands.registerCommand"><code>commands.registerCommand</code></a> <a href="/api/references/vscode-api"><strong>VS Code API</strong></a> to bind a function to the registered command ID <code>helloworld.helloWorld</code>.</li>
</ul>
<p>Understanding these three concepts is crucial to writing extensions in VS Code:</p>
<ul>
<li><a href="/api/references/activation-events"><strong>Activation Events</strong></a>: events upon which your extension becomes active.</li>
<li><a href="/api/references/contribution-points"><strong>Contribution Points</strong></a>: static declarations that you make in the <code>package.json</code> <a href="#_extension-manifest">Extension Manifest</a> to extend VS Code.</li>
<li><a href="/api/references/vscode-api"><strong>VS Code API</strong></a>: a set of JavaScript APIs that you can invoke in your extension code.</li>
</ul>
<p>In general, your extension would use a combination of Contribution Points and VS Code API to extend VS Code's functionality. The <a href="/api/extension-capabilities/overview">Extension Capabilities Overview</a> topic helps you find the right Contribution Point and VS Code API for your extension.</p>
<p>Let's take a closer look at <code>Hello World</code> sample's source code and see how these concepts apply to it.</p>
<h2 id="extension-file-structure" data-needslink="extension-file-structure">Extension File Structure</h2>
<pre class="shiki" data-lang="text" shiki-themes dark-plus light-plus" style="--shiki-dark:#D4D4D4;--shiki-light:#000000;--shiki-dark-bg:#1E1E1E;--shiki-light-bg:#FFFFFF" tabindex="0"><code><span class="line"><span>.</span></span>
<span class="line"><span>├── .vscode</span></span>
<span class="line"><span>│   ├── launch.json     // Config for launching and debugging the extension</span></span>
<span class="line"><span>│   └── tasks.json      // Config for build task that compiles TypeScript</span></span>
<span class="line"><span>├── .gitignore          // Ignore build output and node_modules</span></span>
<span class="line"><span>├── README.md           // Readable description of your extension's functionality</span></span>
<span class="line"><span>├── src</span></span>
<span class="line"><span>│   └── extension.ts    // Extension source code</span></span>
<span class="line"><span>├── package.json        // Extension manifest</span></span>
<span class="line"><span>├── tsconfig.json       // TypeScript configuration</span></span>
<span class="line"><span></span></span></code></pre>
<p>You can read more about the configuration files:</p>
<ul>
<li><code>launch.json</code> used to configure VS Code <a href="/docs/debugtest/debugging">Debugging</a></li>
<li><code>tasks.json</code> for defining VS Code <a href="/docs/debugtest/tasks">Tasks</a></li>
<li><code>tsconfig.json</code> consult the TypeScript <a href="https://www.typescriptlang.org/docs/handbook/tsconfig-json.html" class="external-link" target="_blank">Handbook</a></li>
</ul>
<p>However, let's focus on <code>package.json</code> and <code>extension.ts</code>, which are essential to understanding the <code>Hello World</code> extension.</p>
<h3 id="extension-manifest" data-needslink="extension-manifest">Extension Manifest</h3>
<p>Each VS Code extension must have a <code>package.json</code> as its <a href="/api/references/extension-manifest">Extension Manifest</a>. The <code>package.json</code> contains a mix of Node.js fields such as <code>scripts</code> and <code>devDependencies</code> and VS Code specific fields such as <code>publisher</code>, <code>activationEvents</code> and <code>contributes</code>. You can find descriptions of all VS Code specific fields in <a href="/api/references/extension-manifest">Extension Manifest Reference</a>. Here are some most important fields:</p>
<ul>
<li><code>name</code> and <code>publisher</code>: VS Code uses <code>&lt;publisher&gt;.&lt;name&gt;</code> as a unique ID for the extension. For example, the Hello World sample has the ID <code>vscode-samples.helloworld-sample</code>. VS Code uses the ID to uniquely identify your extension.</li>
<li><code>main</code>: The extension entry point.</li>
<li><code>activationEvents</code> and <code>contributes</code>: <a href="/api/references/activation-events">Activation Events</a> and <a href="/api/references/contribution-points">Contribution Points</a>.</li>
<li><code>engines.vscode</code>: This specifies the minimum version of VS Code API that the extension depends on.</li>
</ul>
<pre class="shiki" data-lang="json" shiki-themes dark-plus light-plus" style="--shiki-dark:#D4D4D4;--shiki-light:#000000;--shiki-dark-bg:#1E1E1E;--shiki-light-bg:#FFFFFF" tabindex="0"><code><span class="line"><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">{</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">  "name"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"helloworld-sample"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">,</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">  "displayName"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"helloworld-sample"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">,</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">  "description"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"HelloWorld example for VS Code"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">,</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">  "version"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"0.0.1"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">,</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">  "publisher"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"vscode-samples"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">,</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">  "repository"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"https://github.com/microsoft/vscode-extension-samples/helloworld-sample"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">,</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">  "engines"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: {</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">    "vscode"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"^1.51.0"</span></span>
<span class="line"><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">  },</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">  "categories"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: [</span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"Other"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">],</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">  "activationEvents"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: [],</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">  "main"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"./out/extension.js"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">,</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">  "contributes"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: {</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">    "commands"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: [</span></span>
<span class="line"><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">      {</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">        "command"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"helloworld.helloWorld"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">,</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">        "title"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"Hello World"</span></span>
<span class="line"><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">      }</span></span>
<span class="line"><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">    ]</span></span>
<span class="line"><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">  },</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">  "scripts"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: {</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">    "vscode:prepublish"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"npm run compile"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">,</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">    "compile"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"tsc -p ./"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">,</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">    "watch"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"tsc -watch -p ./"</span></span>
<span class="line"><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">  },</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">  "devDependencies"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: {</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">    "@types/node"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"^8.10.25"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">,</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">    "@types/vscode"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"^1.51.0"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">,</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">    "tslint"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"^5.16.0"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">,</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#0451A5">    "typescript"</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">"^3.4.5"</span></span>
<span class="line"><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">  }</span></span>
<span class="line"><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">}</span></span>
<span class="line"></span></code></pre>
<blockquote><p><strong>Note</strong>: If your extension targets a VS Code version prior to 1.74, you must explicitly list <code>onCommand:helloworld.helloWorld</code> in <code>activationEvents</code>.</p>
</blockquote><h2 id="extension-entry-file" data-needslink="extension-entry-file">Extension Entry File</h2>
<p>The extension entry file exports two functions, <code>activate</code> and <code>deactivate</code>. <code>activate</code> is executed when your registered <strong>Activation Event</strong> happens. <code>deactivate</code> gives you a chance to clean up before your extension becomes deactivated. For many extensions, explicit cleanup may not be required, and the <code>deactivate</code> method can be removed. However, if an extension needs to perform an operation when VS Code is shutting down or the extension is disabled or uninstalled, this is the method to do so.</p>
<p>The VS Code extension API is declared in the <a href="https://www.npmjs.com/package/@types/vscode" class="external-link" target="_blank">@types/vscode</a> type definitions. The version of the <code>vscode</code> type definitions is controlled by the value in the <code>engines.vscode</code> field in <code>package.json</code>. The <code>vscode</code> types give you IntelliSense, Go to Definition, and other TypeScript language features in your code.</p>
<pre class="shiki" data-lang="ts" shiki-themes dark-plus light-plus" style="--shiki-dark:#D4D4D4;--shiki-light:#000000;--shiki-dark-bg:#1E1E1E;--shiki-light-bg:#FFFFFF" tabindex="0"><code><span class="line"><span style="--shiki-dark:#6A9955;--shiki-light:#008000">// The module 'vscode' contains the VS Code extensibility API</span></span>
<span class="line"><span style="--shiki-dark:#6A9955;--shiki-light:#008000">// Import the module and reference it with the alias vscode in your code below</span></span>
<span class="line"><span style="--shiki-dark:#C586C0;--shiki-light:#AF00DB">import</span><span style="--shiki-dark:#569CD6;--shiki-light:#0000FF"> *</span><span style="--shiki-dark:#C586C0;--shiki-light:#AF00DB"> as</span><span style="--shiki-dark:#9CDCFE;--shiki-light:#001080"> vscode</span><span style="--shiki-dark:#C586C0;--shiki-light:#AF00DB"> from</span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515"> 'vscode'</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">;</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-dark:#6A9955;--shiki-light:#008000">// this method is called when your extension is activated</span></span>
<span class="line"><span style="--shiki-dark:#6A9955;--shiki-light:#008000">// your extension is activated the very first time the command is executed</span></span>
<span class="line"><span style="--shiki-dark:#C586C0;--shiki-light:#AF00DB">export</span><span style="--shiki-dark:#569CD6;--shiki-light:#0000FF"> function</span><span style="--shiki-dark:#DCDCAA;--shiki-light:#795E26"> activate</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">(</span><span style="--shiki-dark:#9CDCFE;--shiki-light:#001080">context</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">: </span><span style="--shiki-dark:#4EC9B0;--shiki-light:#267F99">vscode</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">.</span><span style="--shiki-dark:#4EC9B0;--shiki-light:#267F99">ExtensionContext</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">) {</span></span>
<span class="line"><span style="--shiki-dark:#6A9955;--shiki-light:#008000">  // Use the console to output diagnostic information (console.log) and errors (console.error)</span></span>
<span class="line"><span style="--shiki-dark:#6A9955;--shiki-light:#008000">  // This line of code will only be executed once when your extension is activated</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#001080">  console</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">.</span><span style="--shiki-dark:#DCDCAA;--shiki-light:#795E26">log</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">(</span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">'Congratulations, your extension "helloworld-sample" is now active!'</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">);</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-dark:#6A9955;--shiki-light:#008000">  // The command has been defined in the package.json file</span></span>
<span class="line"><span style="--shiki-dark:#6A9955;--shiki-light:#008000">  // Now provide the implementation of the command with registerCommand</span></span>
<span class="line"><span style="--shiki-dark:#6A9955;--shiki-light:#008000">  // The commandId parameter must match the command field in package.json</span></span>
<span class="line"><span style="--shiki-dark:#569CD6;--shiki-light:#0000FF">  let</span><span style="--shiki-dark:#9CDCFE;--shiki-light:#001080"> disposable</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000"> = </span><span style="--shiki-dark:#9CDCFE;--shiki-light:#001080">vscode</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">.</span><span style="--shiki-dark:#9CDCFE;--shiki-light:#001080">commands</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">.</span><span style="--shiki-dark:#DCDCAA;--shiki-light:#795E26">registerCommand</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">(</span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">'helloworld.helloWorld'</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">, () </span><span style="--shiki-dark:#569CD6;--shiki-light:#0000FF">=></span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000"> {</span></span>
<span class="line"><span style="--shiki-dark:#6A9955;--shiki-light:#008000">    // The code you place here will be executed every time your command is executed</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-dark:#6A9955;--shiki-light:#008000">    // Display a message box to the user</span></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#001080">    vscode</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">.</span><span style="--shiki-dark:#9CDCFE;--shiki-light:#001080">window</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">.</span><span style="--shiki-dark:#DCDCAA;--shiki-light:#795E26">showInformationMessage</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">(</span><span style="--shiki-dark:#CE9178;--shiki-light:#A31515">'Hello World!'</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">);</span></span>
<span class="line"><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">  });</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-dark:#9CDCFE;--shiki-light:#001080">  context</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">.</span><span style="--shiki-dark:#9CDCFE;--shiki-light:#001080">subscriptions</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">.</span><span style="--shiki-dark:#DCDCAA;--shiki-light:#795E26">push</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">(</span><span style="--shiki-dark:#9CDCFE;--shiki-light:#001080">disposable</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">);</span></span>
<span class="line"><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">}</span></span>
<span class="line"></span>
<span class="line"><span style="--shiki-dark:#6A9955;--shiki-light:#008000">// this method is called when your extension is deactivated</span></span>
<span class="line"><span style="--shiki-dark:#C586C0;--shiki-light:#AF00DB">export</span><span style="--shiki-dark:#569CD6;--shiki-light:#0000FF"> function</span><span style="--shiki-dark:#DCDCAA;--shiki-light:#795E26"> deactivate</span><span style="--shiki-dark:#D4D4D4;--shiki-light:#000000">() {}</span></span>
<span class="line"></span></code></pre>

				<div class="feedback" data-edit-url="https://vscode.dev/github/microsoft/vscode-docs/blob/main/api/get-started/extension-anatomy.md"></div>
				
				<div class="body-footer">5/13/2026</div>
				
            </main>
            
            <!-- Right sidebar - On This Page -->
            <aside class="docs-right-sidebar hidden-xs">
                <div class="docs-markdown-actions">
                    <div class="docs-markdown-dropdown" data-raw-url="/raw/api/get-started/extension-anatomy.md">
                        <button type="button" class="docs-markdown-btn-main" data-action="copy-markdown" aria-label="Copy as Markdown">
                            <span class="codicon codicon-copy" aria-hidden="true"></span>
                            <span>Copy as Markdown</span>
                        </button>
                        <button type="button" class="docs-markdown-btn-trigger" aria-haspopup="true" aria-expanded="false" aria-label="More Markdown options">
                            <span class="codicon codicon-chevron-down docs-markdown-chevron" aria-hidden="true"></span>
                        </button>
                        <ul class="docs-markdown-menu" role="menu" aria-label="Markdown options">
                            <li role="menuitem" tabindex="0" data-action="copy-markdown">
                                <span class="codicon codicon-copy" aria-hidden="true"></span>
                                <span>Copy as Markdown</span>
                            </li>
                            <li role="menuitem" tabindex="0" data-action="view-markdown">
                                <span class="codicon codicon-file" aria-hidden="true"></span>
                                <span>View as Markdown</span>
                                <span class="codicon codicon-link-external" aria-hidden="true"></span>
                            </li>
                        </ul>
                    </div>
                </div>
                <nav id="docs-subnavbar" aria-label="On Page">
                    
                    <h4><span class="sr-only">On this page there are 2 sections</span><span
                            aria-hidden="true">On this page</span></h4>
                    <ul class="nav">
                        
                        <li><a href="#extension-file-structure">Extension File Structure</a></li>
                        
                        <li><a href="#extension-entry-file">Extension Entry File</a></li>
                        
                    </ul>
                    
                </nav>
            </aside>
		</div>
		<div class="docs-mobile-widgets visible-xs">
			<div class="connect-widget"></div>
		</div>
	</div>
</div>
		</main>
	</div>

	<div id="search-popup-overlay" class="search-popup-overlay" role="dialog" aria-modal="true" aria-label="Search">
		<div class="search-popup-container">
			<div class="search-popup-header">
				<form class="search-popup-form">
					<div class="input-group">
						<input type="text" name="q" class="search-box form-control" placeholder="Search the website"
							aria-label="Search text" role="combobox" aria-expanded="false"
							aria-autocomplete="list" aria-controls="search-results-listbox"
							aria-activedescendant="" />
						<span class="input-group-btn">
							<button tabindex="0" class="btn" type="submit" aria-label="Search">
								<img class="search-icon-dark" src="/assets/icons/search-dark.svg" alt="Search" />
								<img class="search-icon-light" src="/assets/icons/search.svg" alt="Search" />
							</button>
						</span>
					</div>
				</form>
				<button class="search-popup-close" type="button" aria-label="Close search"><i class="codicon codicon-close" aria-hidden="true"></i></button>
			</div>
			<div class="search-popup-results">
				<ul class="search-popup-results-list" id="search-results-listbox" role="listbox" aria-label="Search results"></ul>
			</div>
			<div class="sr-only" role="status" aria-live="polite" aria-atomic="true" id="search-popup-status"></div>
		</div>
	</div>


	<footer role="contentinfo" class="container">
		<div class="footer-container">
			<div class="footer-row">
				<div class="footer-social">
					<ul class="links">
						<li>
							<a href="https://github.com/microsoft/vscode"><img src="/assets/icons/github-icon.svg" alt="VS Code on Github"></a>
						</li>
						<li>
							<a href="https://go.microsoft.com/fwlink/?LinkID=533687"><img src="/assets/icons/x-icon.svg" class="x-icon" alt="Follow us on X"></a>
						</li>
						<li>
							<a href="https://www.linkedin.com/showcase/vs-code"><img src="/assets/icons/linkedin-icon.svg" alt="VS Code on LinkedIn"></a>
						</li>
						<li>
							<a href="https://bsky.app/profile/vscode.dev"><img src="/assets/icons/bluesky-icon.svg" alt="VS Code on Bluesky"></a>
						</li>
						<li>
							<a href="https://www.reddit.com/r/vscode/"><img src="/assets/icons/reddit-icon.svg" alt="Join the VS Code community on Reddit"></a>
						</li>
						<li>
							<a href="https://www.vscodepodcast.com"><img src="/assets/icons/podcast-icon.svg" alt="The VS Code Insiders Podcast"></a>
						</li>
						<li>
							<a href="https://www.tiktok.com/@vscode"><img src="/assets/icons/tiktok-icon.svg" alt="VS Code on TikTok"></a>
						</li>
						<li>
							<a href="https://www.youtube.com/@code"><img src="/assets/icons/youtube-icon.svg" alt="VS Code on YouTube"></a>
						</li>
						<script>
							function manageConsent() {
								if (siteConsent && siteConsent.isConsentRequired) {
									siteConsent.manageConsent();
								}
							}
						</script>
					</ul>
					<a id="footer-microsoft-link" class="microsoft-logo" href="https://www.microsoft.com">
						<img src="/assets/icons/microsoft.svg" alt="Microsoft homepage" />
					</a>
				</div>
			</div>
			<div class="footer-row">
				<ul class="links">
					<li><a id="footer-support-link" href="https://support.serviceshub.microsoft.com/supportforbusiness/create?sapId=d66407ed-3967-b000-4cfb-2c318cad363d"
						target="_blank" rel="noopener" title="Get support for VS Code"
						aria-label="Get support for VS Code (opens in new tab)">Support</a></li>
					<li><a id="footer-privacy-link" href="https://go.microsoft.com/fwlink/?LinkId=521839"
						target="_blank" rel="noopener" title="View the Microsoft privacy statement"
						aria-label="Microsoft privacy statement (opens in new tab)">Privacy</a></li>
					<li style="display: none;"><a id="footer-cookie-link" style="cursor: pointer;" onclick="manageConsent()"
						target="_blank" rel="noopener">Manage Cookies</a></li>
					<li><a id="footer-terms-link" href="https://www.microsoft.com/legal/terms-of-use"
						target="_blank" rel="noopener" title="View the Microsoft Terms of Use"
						aria-label="Microsoft Terms of Use (opens in new tab)">Terms of Use</a></li>
					<li><a id="footer-license-link" href="/License"
						target="_blank" rel="noopener" title="View the Visual Studio Code license"
						aria-label="Visual Studio Code license (opens in new tab)">License</a></li>
				</ul>
			</div>
			<div class="footer-row">
				<ul class="links">
					<li>
						<svg class="privacy-choices" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 30 14" xml:space="preserve" height="16" width="43">
							<title>Your Privacy Choices Opt-Out Icon</title>
							<path d="M7.4 12.8h6.8l3.1-11.6H7.4C4.2 1.2 1.6 3.8 1.6 7s2.6 5.8 5.8 5.8z" style="fill-rule:evenodd;clip-rule:evenodd;fill:#fff"></path>
							<path d="M22.6 0H7.4c-3.9 0-7 3.1-7 7s3.1 7 7 7h15.2c3.9 0 7-3.1 7-7s-3.2-7-7-7zm-21 7c0-3.2 2.6-5.8 5.8-5.8h9.9l-3.1 11.6H7.4c-3.2 0-5.8-2.6-5.8-5.8z" style="fill-rule:evenodd;clip-rule:evenodd;fill:#06f"></path>
							<path d="M24.6 4c.2.2.2.6 0 .8L22.5 7l2.2 2.2c.2.2.2.6 0 .8-.2.2-.6.2-.8 0l-2.2-2.2-2.2 2.2c-.2.2-.6.2-.8 0-.2-.2-.2-.6 0-.8L20.8 7l-2.2-2.2c-.2-.2-.2-.6 0-.8.2-.2.6-.2.8 0l2.2 2.2L23.8 4c.2-.2.6-.2.8 0z" style="fill:#fff"></path>
							<path d="M12.7 4.1c.2.2.3.6.1.8L8.6 9.8c-.1.1-.2.2-.3.2-.2.1-.5.1-.7-.1L5.4 7.7c-.2-.2-.2-.6 0-.8.2-.2.6-.2.8 0L8 8.6l3.8-4.5c.2-.2.6-.2.9 0z" style="fill:#06f"></path>
						</svg>
						<a id="footer-privacy-choices-link" href="https://aka.ms/YourCaliforniaPrivacyChoices"
						target="_blank" rel="noopener" title="View Your Privacy Choices"
						aria-label="Your Privacy Choices (opens in new tab)">Your Privacy Choices</a></li>
					<li><a id="footer-consumer-health-privacy-link" href="https://go.microsoft.com/fwlink/?linkid=2259814"
						target="_blank" rel="noopener" title="View the Microsoft Consumer Health Privacy policy"
						aria-label="Microsoft Consumer Health Privacy policy (opens in new tab)">Consumer Health Privacy</a></li>
				</ul>
			</div>
		</div>
	</footer>
	<script type="module">
		document.addEventListener('DOMContentLoaded', () => {
			const copilotDeepLinks = document.querySelectorAll('.copilot-deep-link');
			if (copilotDeepLinks.length === 0) {
				return;
			}
			if (window.innerWidth < 992) {
				for (const link of copilotDeepLinks) {
					link.href = 'https://aka.ms/vscode-activatecopilotfree';
				}
			}
		});
	</script>

	<script src="/dist/index.js"></script>

	

	<script type="application/ld+json">
		{
			"@context" : "http://schema.org",
			"@type" : "SoftwareApplication",
			"name" : "Visual Studio Code",
			"softwareVersion": "1.120",
			"offers": {
				"@type": "Offer",
				"price": "0",
				"priceCurrency": "USD"
			},
			"applicationCategory": "DeveloperApplication",
			"applicationSubCategory": "Text Editor",
			"alternateName": "VS Code",
			"datePublished": "2021-11-03",
			"operatingSystem": "Mac, Linux, Windows",
			"logo": "https://code.visualstudio.com/assets/apple-touch-icon.png",
			"screenshot": "https://code.visualstudio.com/assets/images/product-screenshot.png",
			"releaseNotes": "https://code.visualstudio.com/updates",
			"downloadUrl": "https://code.visualstudio.com/download",
			"license": "https://code.visualstudio.com/license",
			"softwareRequirements": "https://code.visualstudio.com/docs/supporting/requirements",
			"url" : "https://code.visualstudio.com",
			"author": {
				"@type": "Organization",
				"name": "Microsoft"
			},
			"publisher": {
				"@type": "Organization",
				"name": "Microsoft"
			},
			"maintainer": {
				"@type": "Organization",
				"name": "Microsoft"
			},
			"potentialAction": {
				"@type": "SearchAction",
				"target": "https://code.visualstudio.com/Search?q={search_term_string}",
				"query-input": "required name=search_term_string"
			},
			"sameAs" : [
				"https://en.wikipedia.org/wiki/Visual_Studio_Code",
				"https://twitter.com/code",
				"https://www.youtube.com/code",
				"https://www.tiktok.com/@vscode",
				"https://github.com/microsoft/vscode"
			]
		}
	</script>
</body>

</html>