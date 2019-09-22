/*
#tabs=3s
*/

function setBase(value)
{
   document.baseAddr = value;
}

function getBase()
{
   if (document.baseAddr == undefined)
      return 0;
   return document.baseAddr;
}

function relAddr(value)
{
   document.write(getBase() + value)
}

var qsParm = new Array();
function qs()
{
   var query = window.location.search.substring(1);
   var parms = query.split('&');
   for (var i=0; i<parms.length; i++) {
      var pos = parms[i].indexOf('=');
      if (pos > 0) {
         var key = parms[i].substring(0,pos);
         var val = parms[i].substring(pos+1);
         qsParm[key] = val;
      }
   }
}

function initBase()
{
   qsParm["dmpbase"] = "0";
   qs();
   setBase(parseInt(qsParm["dmpbase"]));
}

function showparms()
{
   var sub;
   for (sub in qsParm) {
      document.write(qsParm[sub] + "<br/>");
   }
}

function addlink(baseurl, offset, text)
{
   var alink;
   alink = baseurl + "?dmpbase=" + String(offset + getBase());
   alink = "<a href=\"" + alink + "\">" + text + "</a>";
   document.write(alink);
}

function loadtoc()
{
   var homeref;
   var tocref;
   var pathend;

   homeref = location.href;
   pathend = homeref.lastIndexOf("/") + 1
   tocref = homeref.substring(0, pathend) + "toc_" + homeref.substring(pathend)

   parent.tocframe.location.href = tocref;
}
