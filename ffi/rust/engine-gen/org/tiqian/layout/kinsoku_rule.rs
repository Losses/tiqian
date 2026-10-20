use crate::org::tiqian::clreq::clreq_punctuation_policies::ClreqPunctuationPolicies;
use crate::org::tiqian::clreq::kinsoku_level::KinsokuLevel;
use crate::org::tiqian::core::cluster::Cluster;
use crate::runtime::u_string;
use crate::runtime::u_string::UString;


pub trait KinsokuRule: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn KinsokuRule>;
    fn forbidden_at_line_start(&self, cluster: Cluster) -> bool;
    fn forbidden_at_line_end(&self, cluster: Cluster) -> bool;
}

impl Clone for Box<dyn KinsokuRule> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn KinsokuRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}

#[derive(Clone, PartialEq)]
pub struct ClreqKinsokuRule {
    pub(crate) level: KinsokuLevel,
}

impl ClreqKinsokuRule {
    pub fn new(level: Option<KinsokuLevel>) -> Self {
        let level = level.unwrap_or_else(|| KinsokuLevel::Basic);
        Self {
            level: level,
        }
    }

    pub fn to_string(&self) -> UString {
        return UString::from(format!("{}", { let mut __s = UString::new(); __s += &(UString::from("ClreqKinsokuRule(level=")); __s += UString::from(self.level.name()).as_ustr(); __s += &(UString::from(")")); __s }).as_str());
    }

    pub fn forbidden_at_line_start(&self, cluster: Cluster) -> bool {
        if u_string::unit_count(&((cluster.display_text).to_ustring())) == 0 {
            return false;
        }
        return ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(u_string::substring(&(cluster.display_text).to_ustring(), 0i32, 1i32).as_ustr(), self.level);
    }

    pub fn forbidden_at_line_end(&self, cluster: Cluster) -> bool {
        if u_string::unit_count(&((cluster.display_text).to_ustring())) == 0 {
            return false;
        }
        return ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(u_string::substring(&(cluster.display_text).to_ustring(), 0i32, 1i32).as_ustr(), self.level);
    }
}

impl KinsokuRule for ClreqKinsokuRule {
    fn __haxe_type_name(&self) -> &'static str {
        "org.tiqian.layout.KinsokuRule.ClreqKinsokuRule"
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn clone_box(&self) -> Box<dyn KinsokuRule> {
        Box::new(self.clone())
    }

    fn forbidden_at_line_start(&self, cluster: Cluster) -> bool {
        if u_string::unit_count(&((cluster.display_text).to_ustring())) == 0 {
            return false;
        }
        return ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_start(u_string::substring(&(cluster.display_text).to_ustring(), 0i32, 1i32).as_ustr(), self.level);
    }

    fn forbidden_at_line_end(&self, cluster: Cluster) -> bool {
        if u_string::unit_count(&((cluster.display_text).to_ustring())) == 0 {
            return false;
        }
        return ClreqPunctuationPolicies::clreq_punctuation_policies_forbidden_at_line_end(u_string::substring(&(cluster.display_text).to_ustring(), 0i32, 1i32).as_ustr(), self.level);
    }
}
