# Yamete Kudasai Engine

<p>
  <a href="https://www.linkedin.com/in/yairama/" rel="nofollow noreferrer">
    <img src="https://i.stack.imgur.com/gVE0j.png" alt="linkedin" class="icon" width="20" height="20"> LinkedIn
  </a> &nbsp; 
  <a href="https://github.com/Yairama" rel="nofollow noreferrer">
    <img src="https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png" alt="github" class="icon" width="20" height="20"> Github
  </a> &nbsp; 
  <a href="https://gitlab.com/Yairama" rel="nofollow noreferrer">
    <img src="https://cdn-icons-png.flaticon.com/512/5968/5968853.png" alt="gitlab" class="icon" width="20" height="20"> Gitlab
  </a>
</p>


A Graphic engine for personal use

The objective of this project was to create a graphic engine that would allow me to plot the drill holes and test pits of a mining project in the form of cubes, so that each cube represents a specific area and its color represents a range of material thin.

The engine requires a csv file with the data of the drill holes and boreholes, this file is not included in the project for security reasons.

In the *blocks_from_csv.rs* file is the csv format. 


    let df = LazyCsvReader::new(filepath)
        .has_header(true).with_ignore_parser_errors(true)
        .finish()
        .unwrap()
        .select([col("X").cast(DataType::Float32),
            col("Y").cast(DataType::Float32),
            col("Z").cast(DataType::Float32),
            col("Index").cast(DataType::Utf8),
            col("COLOR").cast(DataType::Utf8),
            col("FINO").cast(DataType::Float32),
        ])
        .collect()
        .unwrap();

This is:

https://user-images.githubusercontent.com/45445692/230262057-b609d2af-b562-4926-be97-0fd0747ee472.mp4


