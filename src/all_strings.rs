pub mod strings {
    pub const GITIGNORE_CONTENT: &str = r#"# dependencies
node_modules

# local env files
.env*.local
.env"#;

    pub const ENV_CONTENT: &str = "ATLAS_USERNAME = your own ATLAS username\nATLAS_PASSWORD = your own ATLAS password";

    pub const MONGODB_CONTENT: &str = r#"require("dotenv").config();
const mongoose = require("mongoose");

mongoose
  .connect(
    // write your own ATLAS or Compass server connection string
    `mongodb+srv://${process.env.ATLAS_USERNAME}:${process.env.ATLAS_PASSWORD}@cluster11.gd9qm3q.mongodb.net/student_db?retryWrites=true&w=majority`
  )
  .then(() => {
    console.log("Connected To Database");
  })
  .catch((err) => {
    console.log(err);
  });"#;

    pub const STUDENT_MODEL_CONTENT: &str = r#"const mongoose = require("mongoose");
const studentSchema = new mongoose.Schema(
  {
    name: {
      type: String,
      required: true,
    },
    email: {
      type: String,
      required: true,
      unique: true,
    },
    phone: {
      type: String,
    },
  },
  { timestamps: true }
);

const Student = mongoose.model("Student", studentSchema);
module.exports = { Student };"#;

    pub const STUDENT_ROUTE_CONTENT: &str = r#"const { Student } = require("../models/studentModel");
const express = require("express");
const router = new express.Router();

router.post("/", (req, res) => {
  
  const user = new Student(req.body);
  user
    .save()
    .then(() => {
      res.status(201).send("Student successfully added...");
    })
    .catch((err) => {
      res.status(404).send(err);
    });
});

module.exports = router;"#;

    pub const HELLO_WORLD_ROUTE_CONTENT: &str = r#"const express = require("express");
const router = new express.Router();

router.get('/', (req, res) => {
  res.send('Hello World!');
});


module.exports = router;"#;

    pub const SERVER_CONTENT: &str = r#"require("../connection/mongoDB");

const set_studentRouter = require("../routers/setStudentRoute");
const get_hello_world = require("../routers/getHelloWorldRoute");

const express = require("express");
const cors = require("cors");
const app = express();
const port = 3000;

app.use(express.json());
app.use(
  cors({
    origin: "*",
  })
);

app.use("/student/add" , set_studentRouter);
app.use("/" , get_hello_world);


app.get("*", (req, res) => {
    res.send("sorry this page does not exists.");
});

app.listen(port, () => {
  console.log(`Server is listening at http://localhost:${port}`);
});"#;
}
