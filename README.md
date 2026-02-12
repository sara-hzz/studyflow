StudyFlow

A simple rule-based file organizer for students.



Why I built this

As a student, my Downloads folder was always messy — lecture slides, exam PDFs, CV files, certificates, random images… everything mixed together.

Organizing them manually takes time, especially during exam periods.

So I built StudyFlow to automate this process.



What it does

StudyFlow is a command-line tool written in Rust.

It scans a folder (for example, Downloads), checks file names against keywords defined in a rules.txt file, and then moves the files into structured folders.



If a file matches a course rule, it goes into:

Study/CourseName/FileType/


If it doesn’t match anything, it goes into:

Study/Unsorted/




Features

Custom rules file (rules.txt)

Automatic folder creation

File type detection (PDFs, Images, Archives, etc.)

Duplicate name handling

Dry-run mode (preview changes before moving files)

Move and skip counters



Example rules.txt
Health_Informatics: HI, Medicine, Musculoskeletal
Foundations_of_Science: FoS, Physics, Unit
CV_Job: lebenslauf, arbeitszeugnis
Certificates_Admin: Mark Sheet, Betreff



How to run

Preview mode (recommended first):

cargo run -- ~/Downloads --rules rules.txt --dry-run


Actual execution:

cargo run -- ~/Downloads --rules rules.txt



Who is this for?

University students

Anyone who wants structured academic folders

People who prefer local tools instead of cloud-based automation



Possible future improvements

GUI version

Automatic scheduled runs

Smarter file classification

Pre-built binaries for easier distribution



Author

Sara Zarea
Health Informatics Student
Digital Business Project