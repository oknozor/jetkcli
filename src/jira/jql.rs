pub struct Query {
    pub terms: Vec<(Pair, Keyword)>,
    pub final_term: Pair,
}

impl ToString for Query {
    fn to_string(&self) -> String {
        let final_term = self.final_term.to_string();

        let terms = if !self.terms.is_empty() {
            self.terms
                .iter()
                .map(|(pair, keyword)| format!("{} {} ", pair.to_string(), keyword.name()))
                .collect::<String>()
        } else {
            "".to_string()
        };

        format!("{}{}", terms, final_term)
    }
}

pub struct Pair {
    pub key: Key,
    pub value: String,
    pub operator: Operator,
}

impl ToString for Pair {
    fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            &self.key.name(),
            &self.operator.name(),
            &self.value
        )
    }
}

// https://support.atlassian.com/jira-service-desk-cloud/docs/advanced-search-reference-jql-keywords/
pub enum Keyword {
    And,
    Or,
    Not,
    Empty,
    Null,
    OrderBy,
}

impl Keyword {
    fn name(&self) -> &'static str {
        match self {
            Self::And => "AND",
            Self::Or => "OR",
            Self::Not => "NOT",
            Self::Empty => "EMPTY",
            Self::Null => "NULL",
            Self::OrderBy => "ORDER BY",
        }
    }
}

pub enum Operator {
    Eq,
    Neq,
    Gt,
    GtEq,
    Less,
    LessEq,
    IS,
    Not,
    In,
    NotIn,
}

impl Operator {
    fn name(&self) -> &'static str {
        match self {
            Self::Eq => "=",
            Self::Neq => "!=",
            Self::Gt => ">",
            Self::GtEq => ">=",
            Self::Less => "<",
            Self::LessEq => "<=",
            Self::IS => "IS",
            Self::Not => "NOT",
            Self::In => "IN",
            Self::NotIn => "NOT IN",
        }
    }
}

pub enum Key {
    Status,
    Assignee,
    Project,
}

impl Key {
    fn name(&self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::Assignee => "assignee",
            Self::Project => "project",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pair_to_string() {
        let pair = Pair {
            key: Key::Assignee,
            value: "paul.delafosse".into(),
            operator: Operator::Eq,
        };

        assert_eq!(pair.to_string(), "assignee = paul.delafosse".to_owned())
    }

    #[test]
    fn query_to_string() {
        let pair_1 = Pair {
            key: Key::Assignee,
            value: "paul.delafosse".into(),
            operator: Operator::Eq,
        };

        let pair_2 = Pair {
            key: Key::Project,
            value: "superProject".into(),
            operator: Operator::Eq,
        };

        let query = Query {
            terms: vec![(pair_1, Keyword::And)],
            final_term: pair_2,
        };

        assert_eq!(
            query.to_string(),
            "assignee = paul.delafosse AND project = superProject".to_owned()
        )
    }
}
