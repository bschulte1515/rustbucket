$s={param($s);$sb=[scriptblock]::create($s);sajb $sb -ar "$s";&$sb "$s"};&$s $s
