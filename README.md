# design_project
Senior Design Project Code

# Do all the things:
```

// Install Rust
curl -sSf https://static.rust-lang.org/rustup.sh | sh

// go to Rust Directory
cd server/src/

// Build and run the API server
cargo build
cargo run

// Download node.js
https://nodejs.org/en/

// Check Versions:
node -v
npm -v

// install Meteor
curl https://install.meteor.com/ | sh

// Go to server directory
cd server/
// Start meteor
meteor

// The client should be running now
// at http://localhost:3000

```

Download MongoDB @ ` https://www.mongodb.com/download-center?jmp=nav#community `

Run MongoDB in a separate terminal with `mongod`
Then open another terminal and type `mongo` to interact
with the DB.

Insert these records:
```
db.bs.insert({
	"term":1,	
	"SUBJECT":"BIOE",	
	"NUMBER":"1000",	
	"COURSE_TITLE":"Orientation And Introduction To Bioengineering",	
	"CR_HRS":"3",	
	"COURSE_REQUIREMENTS":"majors only",	
	"COURSE_TYPE":"Major Requirement",	
	"OBOR_TRANSFER_POLICY":""
});

db.courses.insert({
	"ID":"BIOE001",
	"ACADEMIC_PERIOD":"201640",
	"COURSE_LEVEL":"UG",
	"COLLEGE":"EN",
	"COURSE_NUMBER":"1000",
	"COURSE_TITLE_SHORT":"Orient-Intro to BIOE-L1"
});

```