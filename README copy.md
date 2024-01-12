<h4 align="center">
  Hey there, wellcome to my GitHub!
</h4>

<div style="display:flex">


<div style="width:50%;display:inline-block;padding:5px">

<h4>About Me</h4>


```rust
struct Xinchang {
    name:String,
    sex:String,
    current_location:String,
    reserch:Vec<String>,
    program_languages:Vec<String>,
    hobbies:Vec<String>,
}


impl Xinchang {

    fn init() -> Self {
        Xinchang {
            name:String::from("Xinchang Zheng"),
            sex:String::from("Male"),
            current_location:String::from("Houston, Texas"),
            reserch: vec![
                String::from("Bioinformatics & Computational Biology"),
                String::from("Cancer genomics"),
                String::from("Software engineering"),
                String::from("Biomedical database/webserver"),
                String::from("Data visualization"),
                String::from("Long read sequencing")
            ],
            program_languages:vec![
                String::from("Python"),
                String::from("Rust"),
                String::from("R"),
                String::from("JavaScript/HTML/CSS"),
                String::from("Rust"),
                String::from("C/C++"),
                String::from("Shell"),
            ],
            hobbies:vec![
                String::from("Vedio games"),
                String::from("Pokemon!"),
            ]
        }
    }
    fn say_hi()->String{

        "Hey there!".to_string()
    }
}
```
</div>



<div style="width:50%;padding:5px" >
<h4>Activities<h4>
  <div> <img src="https://github-readme-stats.vercel.app/api?username=zhengxinchang&show_icons=true" /> </div>

  <!-- <div>  <img src="https://github-readme-streak-stats.herokuapp.com/?user=zhengxinchang" />  </div> -->

<h4>Programming Languages<h4>
  <div align="center" style="width: 100%;"> 
    <img src="https://github-readme-stats.vercel.app/api/top-langs/?username=zhengxinchang" />
   </div>  

<h4>Recent Activities</h4>
  <div style="width: 100%;"> <img  src="https://github-readme-activity-graph.vercel.app/graph?username=zhengxinchang&theme=xcode" /> </div>

</div>




</div>



<div class="container">


</div>






<style>
  .container {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: strat;
  }
</style>