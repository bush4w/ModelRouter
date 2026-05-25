use regex::Regex;
use crate::models::RoleInfo;

/// Parse Claude.md content to extract role definitions.
/// Supports multiple common formats:
/// 1. Markdown tables with columns: 角色 | 花名 | 寓意
/// 2. Headers with role names like "## 项目经理 (周明)"
/// 3. role-*.md style filenames linked in CLAUDE.md
pub fn parse_claude_md(content: &str) -> Vec<RoleInfo> {
    let mut roles = Vec::new();

    // Strategy 1: Table rows like "| 项目经理 | 周明 | 统筹大局 |"
    let table_re = Regex::new(
        r"\|\s*(.+?)\s*\|\s*(.+?)\s*\|\s*(.+?)\s*\|"
    ).unwrap();

    for cap in table_re.captures_iter(content) {
        let role_name = cap[1].trim().to_string();
        let alias = cap[2].trim().to_string();
        let description = cap[3].trim().to_string();

        // Skip header rows
        if role_name == "角色" || role_name == "花名" || role_name == "---" {
            continue;
        }

        let skills = infer_skills(&role_name);
        roles.push(RoleInfo {
            name: role_name,
            alias: Some(alias),
            description,
            skills,
            file_path: None,
        });
    }

    // Strategy 2: Header-style roles like "## 管理与协调" or "## 研发岗"
    if roles.is_empty() {
        parse_header_roles(content, &mut roles);
    }

    roles
}

fn parse_header_roles(content: &str, roles: &mut Vec<RoleInfo>) {
    // Find sections with role definitions using patterns like:
    // "### 项目经理" or "**项目经理**"
    let section_re = Regex::new(
        r"(?m)(?:###|##)\s+(.+?)\s*\n"
    ).unwrap();

    for cap in section_re.captures_iter(content) {
        let title = cap[1].trim().to_string();
        // Look for role-like names in section headers
        if is_role_name(&title) {
            let description = find_description_after(content, cap.get(0).unwrap().end());
            let skills = infer_skills(&title);
            roles.push(RoleInfo {
                name: title,
                alias: None,
                description,
                skills,
                file_path: None,
            });
        }
    }
}

fn is_role_name(name: &str) -> bool {
    let role_keywords = [
        "经理", "工程师", "设计师", "架构师", "分析", "测试",
        "运维", "安全", "前端", "后端", "数据库", "集成", "商务",
    ];
    role_keywords.iter().any(|kw| name.contains(kw))
}

fn find_description_after(content: &str, start: usize) -> String {
    let rest = &content[start..];
    rest.lines()
        .find(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .unwrap_or("")
        .trim()
        .to_string()
}

fn infer_skills(role_name: &str) -> Vec<String> {
    let skills_map: Vec<(&str, &[&str])> = vec![
        ("项目经理", &["项目管理", "需求分析", "跨部门协调", "风险管理"]),
        ("产品工程师", &["需求分析", "用户研究", "PRD撰写", "数据驱动"]),
        ("产品架构师", &["系统设计", "技术选型", "架构评审", "DDD"]),
        ("UI设计师", &["视觉设计", "交互设计", "Figma", "设计系统"]),
        ("前端工程师", &["React", "TypeScript", "CSS", "性能优化"]),
        ("后端工程师", &["API设计", "数据库", "微服务", "Go/Rust/Java"]),
        ("数据库工程师", &["SQL优化", "数据建模", "索引设计", "数据迁移"]),
        ("外部系统集成工程师", &["API集成", "Webhook", "OAuth2", "数据同步"]),
        ("性能优化工程师", &["性能分析", "缓存策略", "CDN", "负载均衡"]),
        ("安全测试工程师", &["渗透测试", "代码审计", "OWASP", "安全加固"]),
        ("运维工程师", &["CI/CD", "Docker", "K8s", "监控告警"]),
        ("售前工程师", &["方案编写", "技术演示", "竞品分析", "客户沟通"]),
        ("商务经理", &["商业分析", "合作谈判", "市场拓展", "收入模型"]),
    ];

    for (key, skills) in &skills_map {
        if role_name.contains(key) {
            return skills.iter().map(|s| s.to_string()).collect();
        }
    }
    vec!["通用任务处理".to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_table_format() {
        let input = r#"
| 角色 | 花名 | 寓意 |
|------|------|------|
| 项目经理 | 周明 | 统筹大局 |
| 前端工程师 | 叶屏 | 用户界面 |
"#;
        let roles = parse_claude_md(input);
        assert_eq!(roles.len(), 2);
        assert_eq!(roles[0].name, "项目经理");
        assert_eq!(roles[0].alias, Some("周明".to_string()));
        assert_eq!(roles[1].name, "前端工程师");
    }
}
