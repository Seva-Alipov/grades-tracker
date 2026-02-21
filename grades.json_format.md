The below file was AI generated.

# 📊 Grade‑Tracker JSON – How It’s Structured

Below is a **copyable template** that shows how you can organize grades for multiple courses in a single file.  
Replace the angle‑bracketed placeholders (`<…>`) with your own values.

---

## 1️⃣ Top‑Level Array

The entire file is an array (`[ … ]`), each element representing one course.

```json
[
    { /* Course 1 */ },
    { /* Course 2 */ }
]
```

---

## 2️⃣ Course Object Schema

| Key           | Type   | Description                                         |
|---------------|--------|-----------------------------------------------------|
| `Course code` | string | Unique identifier, e.g., `"ECE568"`.              |
| `Course name` | string | Full course title, e.g., `"Computer Security"`.    |
| `Semester`    | string | Short semester tag, e.g., `"W26"` or `"F25"`.      |
| `Deliverables`| array  | List of assignments for that course.                |

---

## 3️⃣ Deliverable (Assignment) Object

| Key     | Type   | Description                                                                                       |
|---------|--------|---------------------------------------------------------------------------------------------------|
| `name`  | string | Human‑readable name (`"Labs"`, `"Midterm"`).                                                      |
| `weight`| number | Weight in the final grade (decimal between 0 and 1, e.g., `0.0625` for 6.25 %).                     |
| `grades`| array* | **Optional** – numeric grades or `null`.  
           *If omitted → “not yet graded”.  
           Any `null`s are ignored when computing an average.*

---

## 📋 Full Generic Example

```json
[
    {
        "Course code": "<COURSE_CODE>",
        "Course name": "<COURSE_NAME>",
        "Semester": "<SEMESTER_IDENTIFIER>",
        "Deliverables": [
            {
                "name": "<ASSIGNMENT_NAME>",
                "weight": <WEIGHT>,          // e.g., 0.0625 for 6.25 %
                "grades": [<GRADE1>, <GRADE2>, ...]   // optional
            },
            {
                "name": "<ANOTHER_ASSIGNMENT>",
                "weight": <WEIGHT>
                // omit "grades" if not yet graded
            }
        ]
    },

    {
        "Course code": "<COURSE_CODE_2>",
        "Course name": "<COURSE_NAME_2>",
        "Semester": "<SEMESTER_IDENTIFIER_2>",
        "Deliverables": [
            {
                "name": "<ASSIGNMENT_NAME>",
                "weight": <WEIGHT>
            }
            // …more assignments
        ]
    }

    // …add more courses as needed
]
```

> **How to use this template**  
> 1. Copy the block above into a file named `grades.json` (or any name you prefer).  
> 2. Replace each `<…>` placeholder with your own values, keeping the surrounding JSON syntax intact.