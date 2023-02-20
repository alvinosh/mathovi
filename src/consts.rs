pub const PREFIX: &'static str = r#"
\documentclass{article}

\pagestyle{empty}

\usepackage[a6paper, margin={2cm,2cm},twocolumn, layouthoffset=0pt]{geometry}

\usepackage[utf8]{inputenc}
\usepackage{lmodern}
\usepackage{amssymb}

\begin{document}
"#;

pub const SUFFIX: &'static str = r#"
\end{document}
"#;
