    
// id is {{ id }}

//<script src="https://ajax.googleapis.com/ajax/libs/jquery/3.7.1/jquery.min.js"></script>
// <script> 



const display_flowcharts = (id,wanted,number_of_frames) => {
    $("details").open = true ;
    for (let i=1;i<=number_of_frames;i++) {
        let id_full = id+"-"+i
        let div = document.getElementById(id_full);
        if (i==wanted) {
            console.log("show "+id) ;
            div.hidden=false ;
            div.visibility="visible" ;
        }
        else {
            // console.log("hide") ;
            div.hidden=true; 
            div.visibility="hidden" ;
        }
    }
};