<!DOCTYPE html>





<html lang="en">






<head>
  <script type="text/javascript">
    
    const currentTheme = localStorage.getItem("theme")
    if (currentTheme) {
      
      const prefersDarkScheme = window.matchMedia("(prefers-color-scheme: dark)").matches
      if (prefersDarkScheme === (currentTheme === "dark")) localStorage.removeItem("theme")
      else if ((prefersDarkScheme && currentTheme === "light")
        || (!prefersDarkScheme && currentTheme === "dark")) {
	document.documentElement.dataset.theme = currentTheme
      }
    }
  </script>

  <meta charset='utf-8'>
  <meta content='IE=edge,chrome=1' http-equiv='X-UA-Compatible'>
  <meta name="viewport" content="width=device-width, initial-scale=1">

  <title>Git - Git Objects</title>

  <link href="/favicon.ico" rel='shortcut icon' type='image/x-icon'>

  
  <link rel="stylesheet" href="/application.min.cfad2cd1f435c38cb55e84c374bc4b8323a290eaafbdb74986a28ed0c2064cf8.css" integrity="sha256-z60s0fQ1w4y1XoTDdLxLgyOikOqvvbdJhqKO0MIGTPg=">
  <script src="/js/modernizr.js"></script>
  

</head>

<body id="learn">

  <div class="inner">
    <header>
  <a id="logo" href="/"><img src="/images/logo@2x.png" width="110" height="46" alt="Git" class="no-filter" /></a>
  <span id="tagline"></span>
  <script type="text/javascript">
   const taglines = [
     "fast-version-control",
     "everything-is-local",
     "distributed-even-if-your-workflow-isnt",
     "local-branching-on-the-cheap",
     "distributed-is-the-new-centralized"
   ];
   var tagline = taglines[Math.floor(Math.random() * taglines.length)];
   document.getElementById('tagline').innerHTML = '--' + tagline;
  </script>

  
  <div id="search-container">
    <form id="search" action="/search/results">
      <input id="search-text" name="search" placeholder="Type / to search entire site…" autocomplete="off" type="text" />
    </form>
    <div id="search-results"></div>
  </div>
  
  <img src="/images/dark-mode.svg" id="dark-mode-button" />
</header>

  </div> 

  
    
    <div class="inner">
      <div id="content-wrapper">
        
<div tabindex="1" class="sidebar-btn"></div>
<aside class="sidebar" id="sidebar">
  <nav>
    <ul>
      <li>
        <a href="/about">About</a>
        <ul >
          <li>
            <a href="/about/trademark">Trademark</a>
          </li>
        </ul>
      </li>

      <li>
        <a href="/learn" class="active">Learn</a>
        <ul class="expanded">
          <li>
            <a href="/book" class="active">Book</a>
          </li>
          <li>
            <a href="/cheat-sheet">Cheat Sheet</a>
          </li>
          <li>
            <a href="/videos">Videos</a>
          </li>
          <li>
            <a href="/doc/ext">External Links</a>
          </li>
        </ul>
      </li>

      <li>
        <a href="/tools">Tools</a>
        <ul >
          <li>
            <a href="/tools/command-line">Command Line</a>
          </li>
          <li>
            <a href="/tools/guis">GUIs</a>
          </li>
          <li>
            <a href="/tools/hosting">Hosting</a>
          </li>
        </ul>
      </li>
      <li>
        <a href="/docs">Reference</a>
      </li>
      <li>
        <a href="/install" class="install-link">Install</a>
      </li>
      <li>
        <a href="/community">Community</a>
        <ul>
          
        </ul>
      </li>
    </ul>

    
    

    
      <hr class="sidebar">
        
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
          
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
                
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
          
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
              
            
          
        
      
      
<p>
This book is available in
  <a href="/book/en/v2/Git-Internals-Git-Objects">English</a>.
</p>
<p>
  Full translation available in
  <table>
    <tr><td><a href="/book/az/v2/Git%e2%80%99in-Daxili-%c4%b0%c5%9fl%c9%99ri-Git-Obyektl%c9%99ri">azərbaycan dili</a>,</td></tr>
    <tr><td><a href="/book/bg/v2/%d0%9f%d1%80%d0%b8%d0%bb%d0%be%d0%b6%d0%b5%d0%bd%d0%b8%d0%b5-B:-%d0%92%d0%b3%d1%80%d0%b0%d0%b6%d0%b4%d0%b0%d0%bd%d0%b5-%d0%bd%d0%b0-Git-%d0%b2-%d0%bf%d1%80%d0%b8%d0%bb%d0%be%d0%b6%d0%b5%d0%bd%d0%b8%d1%8f-Libgit2">български език</a>,</td></tr>
    <tr><td><a href="/book/de/v2/Git-Interna-Git-Objekte">Deutsch</a>,</td></tr>
    <tr><td><a href="/book/es/v2/Los-entresijos-internos-de-Git-Los-objetos-Git">Español</a>,</td></tr>
    <tr><td><a href="/book/fa/v2/%d9%85%d8%a8%d8%a7%d8%ad%d8%ab-%d8%af%d8%b1%d9%88%d9%86%db%8c-%da%af%db%8c%d8%aa-Git-Internals-%d8%a7%d8%b4%db%8c%d8%a7-%da%af%db%8c%d8%aa-Git-Objects" dir="rtl">فارسی</a>,</td></tr>
    <tr><td><a href="/book/fr/v2/Les-tripes-de-Git-Les-objets-de-Git">Français</a>,</td></tr>
    <tr><td><a href="/book/gr/v2/%ce%95%cf%83%cf%89%cf%84%ce%b5%cf%81%ce%b9%ce%ba%ce%ae-%ce%bb%ce%b5%ce%b9%cf%84%ce%bf%cf%85%cf%81%ce%b3%ce%af%ce%b1-%cf%84%ce%bf%cf%85-Git-%ce%91%ce%bd%cf%84%ce%b9%ce%ba%ce%b5%ce%af%ce%bc%ce%b5%ce%bd%ce%b1-%cf%84%ce%bf%cf%85-Git">Ελληνικά</a>,</td></tr>
    <tr><td><a href="/book/ja/v2/Git%e3%81%ae%e5%86%85%e5%81%b4-Git%e3%82%aa%e3%83%96%e3%82%b8%e3%82%a7%e3%82%af%e3%83%88">日本語</a>,</td></tr>
    <tr><td><a href="/book/ko/v2/Git%ec%9d%98-%eb%82%b4%eb%b6%80-Git-%ea%b0%9c%ec%b2%b4">한국어</a>,</td></tr>
    <tr><td><a href="/book/nl/v2/Git-Binnenwerk-Git-objecten">Nederlands</a>,</td></tr>
    <tr><td><a href="/book/ru/v2/Git-%d0%b8%d0%b7%d0%bd%d1%83%d1%82%d1%80%d0%b8-%d0%9e%d0%b1%d1%8a%d0%b5%d0%ba%d1%82%d1%8b-Git">Русский</a>,</td></tr>
    <tr><td><a href="/book/sl/v2/Notranjost-Gita-Objekti-Git">Slovenščina</a>,</td></tr>
    <tr><td><a href="/book/sr/v2/%d0%93%d0%b8%d1%82-%d0%b8%d0%b7%d0%bd%d1%83%d1%82%d1%80%d0%b0-%d0%93%d0%b8%d1%82-%d0%be%d0%b1%d1%98%d0%b5%d0%ba%d1%82%d0%b8">Српски</a>,</td></tr>
    <tr><td><a href="/book/sv/v2/Git-bakom-kulisserna-Git-objekt">Svenska</a>,</td></tr>
    <tr><td><a href="/book/tl/v2/Mga-Panloob-ng-GIT-Git-Objects">Tagalog</a>,</td></tr>
    <tr><td><a href="/book/tr/v2/Dahili-Git-%c3%96geleri-Git-Nesneleri">Türkçe</a>.</td></tr>
    <tr><td><a href="/book/uk/v2/Git-%d0%b7%d1%81%d0%b5%d1%80%d0%b5%d0%b4%d0%b8%d0%bd%d0%b8-%d0%9e%d0%b1%e2%80%99%d1%94%d0%ba%d1%82%d0%b8-Git">Українська</a>,</td></tr>
    <tr><td><a href="/book/zh/v2/Git-%e5%86%85%e9%83%a8%e5%8e%9f%e7%90%86-Git-%e5%af%b9%e8%b1%a1">简体中文</a>,</td></tr>
  </table>
</p>
<p>
  Partial translations available in
  <table>
    <tr><td><a href="/book/cs/v2/Git-Internals-Git-Objects">Čeština</a>,</td></tr>
    <tr><td><a href="/book/mk/v2/%d0%92%d0%bd%d0%b0%d1%82%d1%80%d0%b5%d1%88%d0%bd%d0%be%d1%81%d1%82%d0%b0-%d0%bd%d0%b0-Git-Git-Objects">Македонски</a>,</td></tr>
    <tr><td><a href="/book/pl/v2/Mechanizmy-wewn%c4%99trzne-w-Git-Obiekty-Gita">Polski</a>,</td></tr>
    <tr><td><a href="/book/pt-br/v2/Funcionamento-Interno-do-Git-Objetos-do-Git">Português (Brasil)</a>,</td></tr>
    <tr><td><a href="/book/uz/v2/Git-Internals-Git-Objects">Ўзбекча</a>,</td></tr>
    <tr><td><a href="/book/zh-tw/v2/Git-Internals-Git-Objects">繁體中文</a>,</td></tr>
  </table>
</p>
<p>
  Translations started for
  <table>
    <tr><td><a href="/book/be/v2/Git-Internals-Git-Objects">Беларуская</a>,</td></tr>
    <tr><td><a href="/book/id/v2/Git-Internals-Git-Objects">Indonesian</a>,</td></tr>
    <tr><td><a href="/book/it/v2/Git-Internals-Git-Objects">Italiano</a>,</td></tr>
    <tr><td><a href="/book/ms/v2/Git-Internals-Git-Objects">Bahasa Melayu</a>,</td></tr>
    <tr><td><a href="/book/pt-pt/v2/Internos-do-Git-Git-Objects">Português (Portugal)</a>.</td></tr>
  </table>
</p>
<hr class="sidebar"/>
<p>
The source of this book is <a href="https://github.com/progit/progit2">hosted on GitHub.</a></br>
Patches, suggestions and comments are welcome.
</p>

    
  </nav>
</aside>

        <div id="content">
          
          <div id="book-chapters">
            <a class="dropdown-trigger" id="book-chapters-trigger" data-panel-id="chapters-dropdown" href="#">Chapters ▾</a>







<div class='dropdown-panel' id='chapters-dropdown'>
  <div class='three-column'>
    <div class="column-left">
      
      
      <ol class='book-toc'>
  
  
  <li class='chapter'>
  <h2>1. <a href="/book/en/v2/Getting-Started-About-Version-Control">Getting Started</a></h2>
    <ol>
      
        <li>
          1.1
          <a href="/book/en/v2/Getting-Started-About-Version-Control">About Version Control</a>
        </li>
      
        <li>
          1.2
          <a href="/book/en/v2/Getting-Started-A-Short-History-of-Git">A Short History of Git</a>
        </li>
      
        <li>
          1.3
          <a href="/book/en/v2/Getting-Started-What-is-Git%3F">What is Git?</a>
        </li>
      
        <li>
          1.4
          <a href="/book/en/v2/Getting-Started-The-Command-Line">The Command Line</a>
        </li>
      
        <li>
          1.5
          <a href="/book/en/v2/Getting-Started-Installing-Git">Installing Git</a>
        </li>
      
        <li>
          1.6
          <a href="/book/en/v2/Getting-Started-First-Time-Git-Setup">First-Time Git Setup</a>
        </li>
      
        <li>
          1.7
          <a href="/book/en/v2/Getting-Started-Getting-Help">Getting Help</a>
        </li>
      
        <li>
          1.8
          <a href="/book/en/v2/Getting-Started-Summary">Summary</a>
        </li>
      
    </ol>
  </li>
  
  
  
  <li class='chapter'>
  <h2>2. <a href="/book/en/v2/Git-Basics-Getting-a-Git-Repository">Git Basics</a></h2>
    <ol>
      
        <li>
          2.1
          <a href="/book/en/v2/Git-Basics-Getting-a-Git-Repository">Getting a Git Repository</a>
        </li>
      
        <li>
          2.2
          <a href="/book/en/v2/Git-Basics-Recording-Changes-to-the-Repository">Recording Changes to the Repository</a>
        </li>
      
        <li>
          2.3
          <a href="/book/en/v2/Git-Basics-Viewing-the-Commit-History">Viewing the Commit History</a>
        </li>
      
        <li>
          2.4
          <a href="/book/en/v2/Git-Basics-Undoing-Things">Undoing Things</a>
        </li>
      
        <li>
          2.5
          <a href="/book/en/v2/Git-Basics-Working-with-Remotes">Working with Remotes</a>
        </li>
      
        <li>
          2.6
          <a href="/book/en/v2/Git-Basics-Tagging">Tagging</a>
        </li>
      
        <li>
          2.7
          <a href="/book/en/v2/Git-Basics-Git-Aliases">Git Aliases</a>
        </li>
      
        <li>
          2.8
          <a href="/book/en/v2/Git-Basics-Summary">Summary</a>
        </li>
      
    </ol>
  </li>
  
  
  
  <li class='chapter'>
  <h2>3. <a href="/book/en/v2/Git-Branching-Branches-in-a-Nutshell">Git Branching</a></h2>
    <ol>
      
        <li>
          3.1
          <a href="/book/en/v2/Git-Branching-Branches-in-a-Nutshell">Branches in a Nutshell</a>
        </li>
      
        <li>
          3.2
          <a href="/book/en/v2/Git-Branching-Basic-Branching-and-Merging">Basic Branching and Merging</a>
        </li>
      
        <li>
          3.3
          <a href="/book/en/v2/Git-Branching-Branch-Management">Branch Management</a>
        </li>
      
        <li>
          3.4
          <a href="/book/en/v2/Git-Branching-Branching-Workflows">Branching Workflows</a>
        </li>
      
        <li>
          3.5
          <a href="/book/en/v2/Git-Branching-Remote-Branches">Remote Branches</a>
        </li>
      
        <li>
          3.6
          <a href="/book/en/v2/Git-Branching-Rebasing">Rebasing</a>
        </li>
      
        <li>
          3.7
          <a href="/book/en/v2/Git-Branching-Summary">Summary</a>
        </li>
      
    </ol>
  </li>
  
  
  
  <li class='chapter'>
  <h2>4. <a href="/book/en/v2/Git-on-the-Server-The-Protocols">Git on the Server</a></h2>
    <ol>
      
        <li>
          4.1
          <a href="/book/en/v2/Git-on-the-Server-The-Protocols">The Protocols</a>
        </li>
      
        <li>
          4.2
          <a href="/book/en/v2/Git-on-the-Server-Getting-Git-on-a-Server">Getting Git on a Server</a>
        </li>
      
        <li>
          4.3
          <a href="/book/en/v2/Git-on-the-Server-Generating-Your-SSH-Public-Key">Generating Your SSH Public Key</a>
        </li>
      
        <li>
          4.4
          <a href="/book/en/v2/Git-on-the-Server-Setting-Up-the-Server">Setting Up the Server</a>
        </li>
      
        <li>
          4.5
          <a href="/book/en/v2/Git-on-the-Server-Git-Daemon">Git Daemon</a>
        </li>
      
        <li>
          4.6
          <a href="/book/en/v2/Git-on-the-Server-Smart-HTTP">Smart HTTP</a>
        </li>
      
        <li>
          4.7
          <a href="/book/en/v2/Git-on-the-Server-GitWeb">GitWeb</a>
        </li>
      
        <li>
          4.8
          <a href="/book/en/v2/Git-on-the-Server-GitLab">GitLab</a>
        </li>
      
        <li>
          4.9
          <a href="/book/en/v2/Git-on-the-Server-Third-Party-Hosted-Options">Third Party Hosted Options</a>
        </li>
      
        <li>
          4.10
          <a href="/book/en/v2/Git-on-the-Server-Summary">Summary</a>
        </li>
      
    </ol>
  </li>
  
  
  
  <li class='chapter'>
  <h2>5. <a href="/book/en/v2/Distributed-Git-Distributed-Workflows">Distributed Git</a></h2>
    <ol>
      
        <li>
          5.1
          <a href="/book/en/v2/Distributed-Git-Distributed-Workflows">Distributed Workflows</a>
        </li>
      
        <li>
          5.2
          <a href="/book/en/v2/Distributed-Git-Contributing-to-a-Project">Contributing to a Project</a>
        </li>
      
        <li>
          5.3
          <a href="/book/en/v2/Distributed-Git-Maintaining-a-Project">Maintaining a Project</a>
        </li>
      
        <li>
          5.4
          <a href="/book/en/v2/Distributed-Git-Summary">Summary</a>
        </li>
      
    </ol>
  </li>
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
</ol>

    </div>
    <div class='column-middle'>
      
      
      <ol class='book-toc'>
  
  
  
  
  
  
  
  
  
  
  
  
  <li class='chapter'>
  <h2>6. <a href="/book/en/v2/GitHub-Account-Setup-and-Configuration">GitHub</a></h2>
    <ol>
      
        <li>
          6.1
          <a href="/book/en/v2/GitHub-Account-Setup-and-Configuration">Account Setup and Configuration</a>
        </li>
      
        <li>
          6.2
          <a href="/book/en/v2/GitHub-Contributing-to-a-Project">Contributing to a Project</a>
        </li>
      
        <li>
          6.3
          <a href="/book/en/v2/GitHub-Maintaining-a-Project">Maintaining a Project</a>
        </li>
      
        <li>
          6.4
          <a href="/book/en/v2/GitHub-Managing-an-organization">Managing an organization</a>
        </li>
      
        <li>
          6.5
          <a href="/book/en/v2/GitHub-Scripting-GitHub">Scripting GitHub</a>
        </li>
      
        <li>
          6.6
          <a href="/book/en/v2/GitHub-Summary">Summary</a>
        </li>
      
    </ol>
  </li>
  
  
  
  <li class='chapter'>
  <h2>7. <a href="/book/en/v2/Git-Tools-Revision-Selection">Git Tools</a></h2>
    <ol>
      
        <li>
          7.1
          <a href="/book/en/v2/Git-Tools-Revision-Selection">Revision Selection</a>
        </li>
      
        <li>
          7.2
          <a href="/book/en/v2/Git-Tools-Interactive-Staging">Interactive Staging</a>
        </li>
      
        <li>
          7.3
          <a href="/book/en/v2/Git-Tools-Stashing-and-Cleaning">Stashing and Cleaning</a>
        </li>
      
        <li>
          7.4
          <a href="/book/en/v2/Git-Tools-Signing-Your-Work">Signing Your Work</a>
        </li>
      
        <li>
          7.5
          <a href="/book/en/v2/Git-Tools-Searching">Searching</a>
        </li>
      
        <li>
          7.6
          <a href="/book/en/v2/Git-Tools-Rewriting-History">Rewriting History</a>
        </li>
      
        <li>
          7.7
          <a href="/book/en/v2/Git-Tools-Reset-Demystified">Reset Demystified</a>
        </li>
      
        <li>
          7.8
          <a href="/book/en/v2/Git-Tools-Advanced-Merging">Advanced Merging</a>
        </li>
      
        <li>
          7.9
          <a href="/book/en/v2/Git-Tools-Rerere">Rerere</a>
        </li>
      
        <li>
          7.10
          <a href="/book/en/v2/Git-Tools-Debugging-with-Git">Debugging with Git</a>
        </li>
      
        <li>
          7.11
          <a href="/book/en/v2/Git-Tools-Submodules">Submodules</a>
        </li>
      
        <li>
          7.12
          <a href="/book/en/v2/Git-Tools-Bundling">Bundling</a>
        </li>
      
        <li>
          7.13
          <a href="/book/en/v2/Git-Tools-Replace">Replace</a>
        </li>
      
        <li>
          7.14
          <a href="/book/en/v2/Git-Tools-Credential-Storage">Credential Storage</a>
        </li>
      
        <li>
          7.15
          <a href="/book/en/v2/Git-Tools-Summary">Summary</a>
        </li>
      
    </ol>
  </li>
  
  
  
  <li class='chapter'>
  <h2>8. <a href="/book/en/v2/Customizing-Git-Git-Configuration">Customizing Git</a></h2>
    <ol>
      
        <li>
          8.1
          <a href="/book/en/v2/Customizing-Git-Git-Configuration">Git Configuration</a>
        </li>
      
        <li>
          8.2
          <a href="/book/en/v2/Customizing-Git-Git-Attributes">Git Attributes</a>
        </li>
      
        <li>
          8.3
          <a href="/book/en/v2/Customizing-Git-Git-Hooks">Git Hooks</a>
        </li>
      
        <li>
          8.4
          <a href="/book/en/v2/Customizing-Git-An-Example-Git-Enforced-Policy">An Example Git-Enforced Policy</a>
        </li>
      
        <li>
          8.5
          <a href="/book/en/v2/Customizing-Git-Summary">Summary</a>
        </li>
      
    </ol>
  </li>
  
  
  
  <li class='chapter'>
  <h2>9. <a href="/book/en/v2/Git-and-Other-Systems-Git-as-a-Client">Git and Other Systems</a></h2>
    <ol>
      
        <li>
          9.1
          <a href="/book/en/v2/Git-and-Other-Systems-Git-as-a-Client">Git as a Client</a>
        </li>
      
        <li>
          9.2
          <a href="/book/en/v2/Git-and-Other-Systems-Migrating-to-Git">Migrating to Git</a>
        </li>
      
        <li>
          9.3
          <a href="/book/en/v2/Git-and-Other-Systems-Summary">Summary</a>
        </li>
      
    </ol>
  </li>
  
  
  
  <li class='chapter'>
  <h2>10. <a href="/book/en/v2/Git-Internals-Plumbing-and-Porcelain">Git Internals</a></h2>
    <ol>
      
        <li>
          10.1
          <a href="/book/en/v2/Git-Internals-Plumbing-and-Porcelain">Plumbing and Porcelain</a>
        </li>
      
        <li>
          10.2
          <a href="/book/en/v2/Git-Internals-Git-Objects" class="active">Git Objects</a>
        </li>
      
        <li>
          10.3
          <a href="/book/en/v2/Git-Internals-Git-References">Git References</a>
        </li>
      
        <li>
          10.4
          <a href="/book/en/v2/Git-Internals-Packfiles">Packfiles</a>
        </li>
      
        <li>
          10.5
          <a href="/book/en/v2/Git-Internals-The-Refspec">The Refspec</a>
        </li>
      
        <li>
          10.6
          <a href="/book/en/v2/Git-Internals-Transfer-Protocols">Transfer Protocols</a>
        </li>
      
        <li>
          10.7
          <a href="/book/en/v2/Git-Internals-Maintenance-and-Data-Recovery">Maintenance and Data Recovery</a>
        </li>
      
        <li>
          10.8
          <a href="/book/en/v2/Git-Internals-Environment-Variables">Environment Variables</a>
        </li>
      
        <li>
          10.9
          <a href="/book/en/v2/Git-Internals-Summary">Summary</a>
        </li>
      
    </ol>
  </li>
  
  
  
  
  
  
  
  
</ol>

    </div>
    <div class='column-right'>
      
      
      <ol class='book-toc'>
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  
  <li class='chapter'>
  <h2>A1. <a href="/book/en/v2/Appendix-A:-Git-in-Other-Environments-Graphical-Interfaces">Appendix A: Git in Other Environments</a></h2>
    <ol>
      
        <li>
          A1.1
          <a href="/book/en/v2/Appendix-A:-Git-in-Other-Environments-Graphical-Interfaces">Graphical Interfaces</a>
        </li>
      
        <li>
          A1.2
          <a href="/book/en/v2/Appendix-A:-Git-in-Other-Environments-Git-in-Visual-Studio">Git in Visual Studio</a>
        </li>
      
        <li>
          A1.3
          <a href="/book/en/v2/Appendix-A:-Git-in-Other-Environments-Git-in-Visual-Studio-Code">Git in Visual Studio Code</a>
        </li>
      
        <li>
          A1.4
          <a href="/book/en/v2/Appendix-A:-Git-in-Other-Environments-Git-in-IntelliJ-/-PyCharm-/-WebStorm-/-PhpStorm-/-RubyMine">Git in IntelliJ / PyCharm / WebStorm / PhpStorm / RubyMine</a>
        </li>
      
        <li>
          A1.5
          <a href="/book/en/v2/Appendix-A:-Git-in-Other-Environments-Git-in-Sublime-Text">Git in Sublime Text</a>
        </li>
      
        <li>
          A1.6
          <a href="/book/en/v2/Appendix-A:-Git-in-Other-Environments-Git-in-Bash">Git in Bash</a>
        </li>
      
        <li>
          A1.7
          <a href="/book/en/v2/Appendix-A:-Git-in-Other-Environments-Git-in-Zsh">Git in Zsh</a>
        </li>
      
        <li>
          A1.8
          <a href="/book/en/v2/Appendix-A:-Git-in-Other-Environments-Git-in-PowerShell">Git in PowerShell</a>
        </li>
      
        <li>
          A1.9
          <a href="/book/en/v2/Appendix-A:-Git-in-Other-Environments-Summary">Summary</a>
        </li>
      
    </ol>
  </li>
  
  
  
  <li class='chapter'>
  <h2>A2. <a href="/book/en/v2/Appendix-B:-Embedding-Git-in-your-Applications-Command-line-Git">Appendix B: Embedding Git in your Applications</a></h2>
    <ol>
      
        <li>
          A2.1
          <a href="/book/en/v2/Appendix-B:-Embedding-Git-in-your-Applications-Command-line-Git">Command-line Git</a>
        </li>
      
        <li>
          A2.2
          <a href="/book/en/v2/Appendix-B:-Embedding-Git-in-your-Applications-Libgit2">Libgit2</a>
        </li>
      
        <li>
          A2.3
          <a href="/book/en/v2/Appendix-B:-Embedding-Git-in-your-Applications-JGit">JGit</a>
        </li>
      
        <li>
          A2.4
          <a href="/book/en/v2/Appendix-B:-Embedding-Git-in-your-Applications-go-git">go-git</a>
        </li>
      
        <li>
          A2.5
          <a href="/book/en/v2/Appendix-B:-Embedding-Git-in-your-Applications-Dulwich">Dulwich</a>
        </li>
      
    </ol>
  </li>
  
  
  
  <li class='chapter'>
  <h2>A3. <a href="/book/en/v2/Appendix-C:-Git-Commands-Setup-and-Config">Appendix C: Git Commands</a></h2>
    <ol>
      
        <li>
          A3.1
          <a href="/book/en/v2/Appendix-C:-Git-Commands-Setup-and-Config">Setup and Config</a>
        </li>
      
        <li>
          A3.2
          <a href="/book/en/v2/Appendix-C:-Git-Commands-Getting-and-Creating-Projects">Getting and Creating Projects</a>
        </li>
      
        <li>
          A3.3
          <a href="/book/en/v2/Appendix-C:-Git-Commands-Basic-Snapshotting">Basic Snapshotting</a>
        </li>
      
        <li>
          A3.4
          <a href="/book/en/v2/Appendix-C:-Git-Commands-Branching-and-Merging">Branching and Merging</a>
        </li>
      
        <li>
          A3.5
          <a href="/book/en/v2/Appendix-C:-Git-Commands-Sharing-and-Updating-Projects">Sharing and Updating Projects</a>
        </li>
      
        <li>
          A3.6
          <a href="/book/en/v2/Appendix-C:-Git-Commands-Inspection-and-Comparison">Inspection and Comparison</a>
        </li>
      
        <li>
          A3.7
          <a href="/book/en/v2/Appendix-C:-Git-Commands-Debugging">Debugging</a>
        </li>
      
        <li>
          A3.8
          <a href="/book/en/v2/Appendix-C:-Git-Commands-Patching">Patching</a>
        </li>
      
        <li>
          A3.9
          <a href="/book/en/v2/Appendix-C:-Git-Commands-Email">Email</a>
        </li>
      
        <li>
          A3.10
          <a href="/book/en/v2/Appendix-C:-Git-Commands-External-Systems">External Systems</a>
        </li>
      
        <li>
          A3.11
          <a href="/book/en/v2/Appendix-C:-Git-Commands-Administration">Administration</a>
        </li>
      
        <li>
          A3.12
          <a href="/book/en/v2/Appendix-C:-Git-Commands-Plumbing-Commands">Plumbing Commands</a>
        </li>
      
    </ol>
  </li>
  
  
</ol>

    </div>
  </div>
</div>


            <span class="light" id="edition">
              2nd Edition
            </span>
          </div>
          <div id="main" data-pagefind-filter="category:book" data-pagefind-meta="category:Book" data-pagefind-weight="0.05" data-pagefind-body class="book edition2">
            <h1>10.2 Git Internals - Git Objects</h1>
            <div>
              <h2 id="_objects">Git Objects</h2>
<div class="paragraph">
<p>Git is a content-addressable filesystem.
Great.
What does that mean?
It means that at the core of Git is a simple key-value data store.
What this means is that you can insert any kind of content into a Git repository, for which Git will hand you back a unique key you can use later to retrieve that content.</p>
</div>
<div class="paragraph">
<p>As a demonstration, let’s look at the plumbing command <code>git hash-object</code>, which takes some data, stores it in your <code>.git/objects</code> directory (the <em>object database</em>), and gives you back the unique key that now refers to that data object.</p>
</div>
<div class="paragraph">
<p>First, you initialize a new Git repository and verify that there is (predictably) nothing in the <code>objects</code> directory:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ git init test
Initialized empty Git repository in /tmp/test/.git/
$ cd test
$ find .git/objects
.git/objects
.git/objects/info
.git/objects/pack
$ find .git/objects -type f</code></pre>
</div>
</div>
<div class="paragraph">
<p>Git has initialized the <code>objects</code> directory and created <code>pack</code> and <code>info</code> subdirectories in it, but there are no regular files.
Now, let’s use <code>git hash-object</code> to create a new data object and manually store it in your new Git database:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ echo 'test content' | git hash-object -w --stdin
d670460b4b4aece5915caf5c68d12f560a9fe3e4</code></pre>
</div>
</div>
<div class="paragraph">
<p>In its simplest form, <code>git hash-object</code> would take the content you handed to it and merely return the unique key that <em>would</em> be used to store it in your Git database.
The <code>-w</code> option then tells the command to not simply return the key, but to write that object to the database.
Finally, the <code>--stdin</code> option tells <code>git hash-object</code> to get the content to be processed from stdin; otherwise, the command would expect a filename argument at the end of the command containing the content to be used.</p>
</div>
<div class="paragraph">
<p>The output from the above command is a 40-character checksum hash.
This is the SHA-1 hash — a checksum of the content you’re storing plus a header, which you’ll learn about in a bit.
Now you can see how Git has stored your data:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ find .git/objects -type f
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4</code></pre>
</div>
</div>
<div class="paragraph">
<p>If you again examine your <code>objects</code> directory, you can see that it now contains a file for that new content.
This is how Git stores the content initially — as a single file per piece of content, named with the SHA-1 checksum of the content and its header.
The subdirectory is named with the first 2 characters of the SHA-1, and the filename is the remaining 38 characters.</p>
</div>
<div class="paragraph">
<p>Once you have content in your object database, you can examine that content with the <code>git cat-file</code> command.
This command is sort of a Swiss army knife for inspecting Git objects.
Passing <code>-p</code> to <code>cat-file</code> instructs the command to first figure out the type of content, then display it appropriately:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ git cat-file -p d670460b4b4aece5915caf5c68d12f560a9fe3e4
test content</code></pre>
</div>
</div>
<div class="paragraph">
<p>Now, you can add content to Git and pull it back out again.
You can also do this with content in files.
For example, you can do some simple version control on a file.
First, create a new file and save its contents in your database:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ echo 'version 1' &gt; test.txt
$ git hash-object -w test.txt
83baae61804e65cc73a7201a7252750c76066a30</code></pre>
</div>
</div>
<div class="paragraph">
<p>Then, write some new content to the file, and save it again:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ echo 'version 2' &gt; test.txt
$ git hash-object -w test.txt
1f7a7a472abf3dd9643fd615f6da379c4acb3e3a</code></pre>
</div>
</div>
<div class="paragraph">
<p>Your object database now contains both versions of this new file (as well as the first content you stored there):</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ find .git/objects -type f
.git/objects/1f/7a7a472abf3dd9643fd615f6da379c4acb3e3a
.git/objects/83/baae61804e65cc73a7201a7252750c76066a30
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4</code></pre>
</div>
</div>
<div class="paragraph">
<p>At this point, you can delete your local copy of that <code>test.txt</code> file, then use Git to retrieve, from the object database, either the first version you saved:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ git cat-file -p 83baae61804e65cc73a7201a7252750c76066a30 &gt; test.txt
$ cat test.txt
version 1</code></pre>
</div>
</div>
<div class="paragraph">
<p>or the second version:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ git cat-file -p 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a &gt; test.txt
$ cat test.txt
version 2</code></pre>
</div>
</div>
<div class="paragraph">
<p>But remembering the SHA-1 key for each version of your file isn’t practical; plus, you aren’t storing the filename in your system — just the content.
This object type is called a <em>blob</em>.
You can have Git tell you the object type of any object in Git, given its SHA-1 key, with <code>git cat-file -t</code>:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ git cat-file -t 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a
blob</code></pre>
</div>
</div>
<div class="sect3">
<h3 id="_tree_objects">Tree Objects</h3>
<div class="paragraph">
<p>The next type of Git object we’ll examine is the <em>tree</em>, which solves the problem of storing the filename and also allows you to store a group of files together.
Git stores content in a manner similar to a UNIX filesystem, but a bit simplified.
All the content is stored as tree and blob objects, with trees corresponding to UNIX directory entries and blobs corresponding more or less to inodes or file contents.
A single tree object contains one or more entries, each of which is the SHA-1 hash of a blob or subtree with its associated mode, type, and filename.
For example, let’s say you have a project where the most-recent tree looks something like:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ git cat-file -p master^{tree}
100644 blob a906cb2a4a904a152e80877d4088654daad0c859      README
100644 blob 8f94139338f9404f26296befa88755fc2598c289      Rakefile
040000 tree 99f1a6d12cb4b6f19c8655fca46c3ecf317074e0      lib</code></pre>
</div>
</div>
<div class="paragraph">
<p>The <code>master^{tree}</code> syntax specifies the tree object that is pointed to by the last commit on your <code>master</code> branch.
Notice that the <code>lib</code> subdirectory isn’t a blob but a pointer to another tree:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ git cat-file -p 99f1a6d12cb4b6f19c8655fca46c3ecf317074e0
100644 blob 47c6340d6459e05787f644c2447d2595f5d3a54b      simplegit.rb</code></pre>
</div>
</div>
<div class="admonitionblock note">
<table>
<tr>
<td class="icon">
<div class="title">Note</div>
</td>
<td class="content">
<div class="paragraph">
<p>Depending on what shell you use, you may encounter errors when using the <code>master^{tree}</code> syntax.</p>
</div>
<div class="paragraph">
<p>In CMD on Windows, the <code>^</code> character is used for escaping, so you have to double it to avoid this: <code>git cat-file -p master^^{tree}</code>.
When using PowerShell, parameters using <code>{}</code> characters have to be quoted to avoid the parameter being parsed incorrectly: <code>git cat-file -p 'master^{tree}'</code>.</p>
</div>
<div class="paragraph">
<p>If you’re using ZSH, the <code>^</code> character is used for globbing, so you have to enclose the whole expression in quotes: <code>git cat-file -p "master^{tree}"</code>.</p>
</div>
</td>
</tr>
</table>
</div>
<div class="paragraph">
<p>Conceptually, the data that Git is storing looks something like this:</p>
</div>
<div class="imageblock">
<div class="content">
<img src="/book/en/v2/images/data-model-1.png" alt="Simple version of the Git data model">
</div>
<div class="title">Figure 173. Simple version of the Git data model</div>
</div>
<div class="paragraph">
<p>You can fairly easily create your own tree.
Git normally creates a tree by taking the state of your staging area or index and writing a series of tree objects from it.
So, to create a tree object, you first have to set up an index by staging some files.
To create an index with a single entry — the first version of your <code>test.txt</code> file — you can use the plumbing command <code>git update-index</code>.
You use this command to artificially add the earlier version of the <code>test.txt</code> file to a new staging area.
You must pass it the <code>--add</code> option because the file doesn’t yet exist in your staging area (you don’t even have a staging area set up yet) and <code>--cacheinfo</code> because the file you’re adding isn’t in your directory but is in your database.
Then, you specify the mode, SHA-1, and filename:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ git update-index --add --cacheinfo 100644 \
  83baae61804e65cc73a7201a7252750c76066a30 test.txt</code></pre>
</div>
</div>
<div class="paragraph">
<p>In this case, you’re specifying a mode of <code>100644</code>, which means it’s a normal file.
Other options are <code>100755</code>, which means it’s an executable file; and <code>120000</code>, which specifies a symbolic link.
The mode is taken from normal UNIX modes but is much less flexible — these three modes are the only ones that are valid for files (blobs) in Git (although other modes are used for directories and submodules).</p>
</div>
<div class="paragraph">
<p>Now, you can use <code>git write-tree</code> to write the staging area out to a tree object.
No <code>-w</code> option is needed — calling this command automatically creates a tree object from the state of the index if that tree doesn’t yet exist:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ git write-tree
d8329fc1cc938780ffdd9f94e0d364e0ea74f579
$ git cat-file -p d8329fc1cc938780ffdd9f94e0d364e0ea74f579
100644 blob 83baae61804e65cc73a7201a7252750c76066a30      test.txt</code></pre>
</div>
</div>
<div class="paragraph">
<p>You can also verify that this is a tree object using the same <code>git cat-file</code> command you saw earlier:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ git cat-file -t d8329fc1cc938780ffdd9f94e0d364e0ea74f579
tree</code></pre>
</div>
</div>
<div class="paragraph">
<p>You’ll now create a new tree with the second version of <code>test.txt</code> and a new file as well:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ echo 'new file' &gt; new.txt
$ git update-index --cacheinfo 100644 \
  1f7a7a472abf3dd9643fd615f6da379c4acb3e3a test.txt
$ git update-index --add new.txt</code></pre>
</div>
</div>
<div class="paragraph">
<p>Your staging area now has the new version of <code>test.txt</code> as well as the new file <code>new.txt</code>.
Write out that tree (recording the state of the staging area or index to a tree object) and see what it looks like:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ git write-tree
0155eb4229851634a0f03eb265b69f5a2d56f341
$ git cat-file -p 0155eb4229851634a0f03eb265b69f5a2d56f341
100644 blob fa49b077972391ad58037050f2a75f74e3671e92      new.txt
100644 blob 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a      test.txt</code></pre>
</div>
</div>
<div class="paragraph">
<p>Notice that this tree has both file entries and also that the <code>test.txt</code> SHA-1 is the “version 2” SHA-1 from earlier (<code>1f7a7a</code>).
Just for fun, you’ll add the first tree as a subdirectory into this one.
You can read trees into your staging area by calling <code>git read-tree</code>.
In this case, you can read an existing tree into your staging area as a subtree by using the <code>--prefix</code> option with this command:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ git read-tree --prefix=bak d8329fc1cc938780ffdd9f94e0d364e0ea74f579
$ git write-tree
3c4e9cd789d88d8d89c1073707c3585e41b0e614
$ git cat-file -p 3c4e9cd789d88d8d89c1073707c3585e41b0e614
040000 tree d8329fc1cc938780ffdd9f94e0d364e0ea74f579      bak
100644 blob fa49b077972391ad58037050f2a75f74e3671e92      new.txt
100644 blob 1f7a7a472abf3dd9643fd615f6da379c4acb3e3a      test.txt</code></pre>
</div>
</div>
<div class="paragraph">
<p>If you created a working directory from the new tree you just wrote, you would get the two files in the top level of the working directory and a subdirectory named <code>bak</code> that contained the first version of the <code>test.txt</code> file.
You can think of the data that Git contains for these structures as being like this:</p>
</div>
<div class="imageblock">
<div class="content">
<img src="/book/en/v2/images/data-model-2.png" alt="The content structure of your current Git data">
</div>
<div class="title">Figure 174. The content structure of your current Git data</div>
</div>
</div>
<div class="sect3">
<h3 id="_git_commit_objects">Commit Objects</h3>
<div class="paragraph">
<p>If you’ve done all of the above, you now have three trees that represent the different snapshots of your project that you want to track, but the earlier problem remains: you must remember all three SHA-1 values in order to recall the snapshots.
You also don’t have any information about who saved the snapshots, when they were saved, or why they were saved.
This is the basic information that the commit object stores for you.</p>
</div>
<div class="paragraph">
<p>To create a commit object, you call <code>commit-tree</code> and specify a single tree SHA-1 and which commit objects, if any, directly preceded it.
Start with the first tree you wrote:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ echo 'First commit' | git commit-tree d8329f
fdf4fc3344e67ab068f836878b6c4951e3b15f3d</code></pre>
</div>
</div>
<div class="admonitionblock note">
<table>
<tr>
<td class="icon">
<div class="title">Note</div>
</td>
<td class="content">
<div class="paragraph">
<p>You will get a different hash value because of different creation time and author data.
Moreover, while in principle any commit object can be reproduced precisely given that data, historical details of this book’s construction mean that the printed commit hashes might not correspond to the given commits.
Replace commit and tag hashes with your own checksums further in this chapter.</p>
</div>
</td>
</tr>
</table>
</div>
<div class="paragraph">
<p>Now you can look at your new commit object with <code>git cat-file</code>:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ git cat-file -p fdf4fc3
tree d8329fc1cc938780ffdd9f94e0d364e0ea74f579
author Scott Chacon &lt;schacon@gmail.com&gt; 1243040974 -0700
committer Scott Chacon &lt;schacon@gmail.com&gt; 1243040974 -0700

First commit</code></pre>
</div>
</div>
<div class="paragraph">
<p>The format for a commit object is simple: it specifies the top-level tree for the snapshot of the project at that point; the parent commits if any (the commit object described above does not have any parents); the author/committer information (which uses your <code>user.name</code> and <code>user.email</code> configuration settings and a timestamp); a blank line, and then the commit message.</p>
</div>
<div class="paragraph">
<p>Next, you’ll write the other two commit objects, each referencing the commit that came directly before it:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ echo 'Second commit' | git commit-tree 0155eb -p fdf4fc3
cac0cab538b970a37ea1e769cbbde608743bc96d
$ echo 'Third commit'  | git commit-tree 3c4e9c -p cac0cab
1a410efbd13591db07496601ebc7a059dd55cfe9</code></pre>
</div>
</div>
<div class="paragraph">
<p>Each of the three commit objects points to one of the three snapshot trees you created.
Oddly enough, you have a real Git history now that you can view with the <code>git log</code> command, if you run it on the last commit SHA-1:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ git log --stat 1a410e
commit 1a410efbd13591db07496601ebc7a059dd55cfe9
Author: Scott Chacon &lt;schacon@gmail.com&gt;
Date:   Fri May 22 18:15:24 2009 -0700

	Third commit

 bak/test.txt | 1 +
 1 file changed, 1 insertion(+)

commit cac0cab538b970a37ea1e769cbbde608743bc96d
Author: Scott Chacon &lt;schacon@gmail.com&gt;
Date:   Fri May 22 18:14:29 2009 -0700

	Second commit

 new.txt  | 1 +
 test.txt | 2 +-
 2 files changed, 2 insertions(+), 1 deletion(-)

commit fdf4fc3344e67ab068f836878b6c4951e3b15f3d
Author: Scott Chacon &lt;schacon@gmail.com&gt;
Date:   Fri May 22 18:09:34 2009 -0700

    First commit

 test.txt | 1 +
 1 file changed, 1 insertion(+)</code></pre>
</div>
</div>
<div class="paragraph">
<p>Amazing.
You’ve just done the low-level operations to build up a Git history without using any of the front end commands.
This is essentially what Git does when you run the <code>git add</code> and <code>git commit</code> commands — it stores blobs for the files that have changed, updates the index, writes out trees, and writes commit objects that reference the top-level trees and the commits that came immediately before them.
These three main Git objects — the blob, the tree, and the commit — are initially stored as separate files in your <code>.git/objects</code> directory.
Here are all the objects in the example directory now, commented with what they store:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ find .git/objects -type f
.git/objects/01/55eb4229851634a0f03eb265b69f5a2d56f341 # tree 2
.git/objects/1a/410efbd13591db07496601ebc7a059dd55cfe9 # commit 3
.git/objects/1f/7a7a472abf3dd9643fd615f6da379c4acb3e3a # test.txt v2
.git/objects/3c/4e9cd789d88d8d89c1073707c3585e41b0e614 # tree 3
.git/objects/83/baae61804e65cc73a7201a7252750c76066a30 # test.txt v1
.git/objects/ca/c0cab538b970a37ea1e769cbbde608743bc96d # commit 2
.git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4 # 'test content'
.git/objects/d8/329fc1cc938780ffdd9f94e0d364e0ea74f579 # tree 1
.git/objects/fa/49b077972391ad58037050f2a75f74e3671e92 # new.txt
.git/objects/fd/f4fc3344e67ab068f836878b6c4951e3b15f3d # commit 1</code></pre>
</div>
</div>
<div class="paragraph">
<p>If you follow all the internal pointers, you get an object graph something like this:</p>
</div>
<div class="imageblock">
<div class="content">
<img src="/book/en/v2/images/data-model-3.png" alt="All the reachable objects in your Git directory">
</div>
<div class="title">Figure 175. All the reachable objects in your Git directory</div>
</div>
</div>
<div class="sect3">
<h3 id="_object_storage">Object Storage</h3>
<div class="paragraph">
<p>We mentioned earlier that there is a header stored with every object you commit to your Git object database.
Let’s take a minute to see how Git stores its objects.
You’ll see how to store a blob object — in this case, the string “what is up, doc?” — interactively in the Ruby scripting language.</p>
</div>
<div class="paragraph">
<p>You can start up interactive Ruby mode with the <code>irb</code> command:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ irb
&gt;&gt; content = "what is up, doc?"
=&gt; "what is up, doc?"</code></pre>
</div>
</div>
<div class="paragraph">
<p>Git first constructs a header which starts by identifying the type of object — in this case, a blob.
To that first part of the header, Git adds a space followed by the size in bytes of the content, and adding a final null byte:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">&gt;&gt; header = "blob #{content.bytesize}\0"
=&gt; "blob 16\u0000"</code></pre>
</div>
</div>
<div class="paragraph">
<p>Git concatenates the header and the original content and then calculates the SHA-1 checksum of that new content.
You can calculate the SHA-1 value of a string in Ruby by including the SHA1 digest library with the <code>require</code> command and then calling <code>Digest::SHA1.hexdigest()</code> with the string:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">&gt;&gt; store = header + content
=&gt; "blob 16\u0000what is up, doc?"
&gt;&gt; require 'digest/sha1'
=&gt; true
&gt;&gt; sha1 = Digest::SHA1.hexdigest(store)
=&gt; "bd9dbf5aae1a3862dd1526723246b20206e5fc37"</code></pre>
</div>
</div>
<div class="paragraph">
<p>Let’s compare that to the output of <code>git hash-object</code>.
Here we use <code>echo -n</code> to prevent adding a newline to the input.</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">$ echo -n "what is up, doc?" | git hash-object --stdin
bd9dbf5aae1a3862dd1526723246b20206e5fc37</code></pre>
</div>
</div>
<div class="paragraph">
<p>Git compresses the new content with zlib, which you can do in Ruby with the zlib library.
First, you need to require the library and then run <code>Zlib::Deflate.deflate()</code> on the content:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">&gt;&gt; require 'zlib'
=&gt; true
&gt;&gt; zlib_content = Zlib::Deflate.deflate(store)
=&gt; "x\x9CK\xCA\xC9OR04c(\xCFH,Q\xC8,V(-\xD0QH\xC9O\xB6\a\x00_\x1C\a\x9D"</code></pre>
</div>
</div>
<div class="paragraph">
<p>Finally, you’ll write your zlib-deflated content to an object on disk.
You’ll determine the path of the object you want to write out (the first two characters of the SHA-1 value being the subdirectory name, and the last 38 characters being the filename within that directory).
In Ruby, you can use the <code>FileUtils.mkdir_p()</code> function to create the subdirectory if it doesn’t exist.
Then, open the file with <code>File.open()</code> and write out the previously zlib-compressed content to the file with a <code>write()</code> call on the resulting file handle:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">&gt;&gt; path = '.git/objects/' + sha1[0,2] + '/' + sha1[2,38]
=&gt; ".git/objects/bd/9dbf5aae1a3862dd1526723246b20206e5fc37"
&gt;&gt; require 'fileutils'
=&gt; true
&gt;&gt; FileUtils.mkdir_p(File.dirname(path))
=&gt; ".git/objects/bd"
&gt;&gt; File.open(path, 'w') { |f| f.write zlib_content }
=&gt; 32</code></pre>
</div>
</div>
<div class="paragraph">
<p>Let’s check the content of the object using <code>git cat-file</code>:</p>
</div>
<div class="listingblock">
<div class="content">
<pre class="highlight"><code class="language-console" data-lang="console">---
$ git cat-file -p bd9dbf5aae1a3862dd1526723246b20206e5fc37
what is up, doc?
---</code></pre>
</div>
</div>
<div class="paragraph">
<p>That’s it – you’ve created a valid Git blob object.</p>
</div>
<div class="paragraph">
<p>All Git objects are stored the same way, just with different types – instead of the string blob, the header will begin with commit or tree.
Also, although the blob content can be nearly anything, the commit and tree content are very specifically formatted.</p>
</div>
</div>
<div id="nav"><a href="/book/en/v2/Git-Internals-Plumbing-and-Porcelain">prev</a> | <a href="/book/en/v2/Git-Internals-Git-References">next</a></div>
            </div>
          </div>
          
        </div>
      </div>
      <footer>
  <div class="site-source">
    <a href="/site">About this site</a><br>
    Patches, suggestions, and comments are welcome.
  </div>
  <div class="sfc-member">
    Git is a member of <a href="/sfc">Software Freedom Conservancy</a>
  </div>
</footer>

<script src="https://code.jquery.com/jquery-3.7.1.min.js"></script>
<script src="https://code.jquery.com/ui/1.14.1/jquery-ui.min.js"></script>
<script src="/js/jquery.defaultvalue.js"></script>
<script src="/js/session.min.js"></script>

<script src="/js/application.min.501227e6642ae966f2b54bebd344b2c06c1eb7080bf76f9297621652492d15fd.js" integrity="sha256-UBIn5mQq6WbytUvr00SywGwetwgL92&#43;Sl2IWUkktFf0="></script>

    </div>
  

</body>
</html>

