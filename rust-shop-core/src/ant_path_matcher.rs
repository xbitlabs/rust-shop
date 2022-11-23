use std::collections::HashMap;
use lazy_static::lazy_static;
use substring::Substring;
use validator::HasLen;

static DEFAULT_PATH_SEPARATOR: &'static str = "/";
const CACHE_TURNOFF_THRESHOLD: u32 = 65536;

lazy_static! {
    pub static ref VARIABLE_PATTERN: pcre2::bytes::Regex = pcre2::bytes::Regex::new(r#"\\{[^/]+?\\}"#);
}

lazy_static! {
    pub static ref WILDCARD_CHARS: Vec<char> = vec!['*', '?', '{'];
}

pub trait PathMatcher {
    fn is_pattern(path: String) -> bool;
    fn is_match(pattern: String, path: String) -> bool;
    fn match_start(pattern: String, path: String) -> bool;
    fn extract_path_within_pattern(pattern: String, path: String) -> String;
    fn extract_uri_template_variables(pattern: String, path: String) -> HashMap<String, String>;
    //fn Comparator<String> getPatternComparator(String path);
    fn combine(pattern1: String, pattern2: String) -> String;
}

struct  PathSeparatorPatternCache {
    ends_on_wild_card:String,
    ends_on_double_wild_card:String,
}
impl PathSeparatorPatternCache{
    pub fn new(path_separator:String ) ->Self {
        PathSeparatorPatternCache{
            ends_on_wild_card: path_separator.clone() + "*",
            ends_on_double_wild_card: path_separator.clone() + "**"
        }
    }
    pub fn get_ends_on_wild_card(&self) ->&String {
        &self.ends_on_wild_card
    }
    pub fn get_ends_on_double_wild_card(&self) ->&String {
        &self.ends_on_double_wild_card
    }
}

lazy_static! {
    pub static ref GLOB_PATTERN: pcre2::bytes::Regex = pcre2::bytes::Regex::new(r#"\\?|\\*|\\{((?:\\{[^/]+?\\}|[^/{}]|\\\\[{}])+?)\\}"#);
}

const  DEFAULT_VARIABLE_PATTERN:&'static str = "((?s).*)";


pub struct AntPathStringMatcher{
    raw_pattern:String,
    case_sensitive:bool,
    exact_match:bool,
    pattern:String,
    variable_names: Vec<String>
}
impl AntPathStringMatcher{
    pub fn new(pattern:String, case_sensitive:bool ) {
        //this.rawPattern = pattern;
        //this.caseSensitive = case_sensitive;
        let mut pattern_builder:String = String::from("");
        let matcher = GLOB_PATTERN.find_iter(pattern.as_ref());
        let mut end = 0;
        while (matcher.find()) {
            pattern_builder.append(quote(pattern, end, matcher.start()));
            String match = matcher.group();
            if ("?".equals(match)) {
                pattern_builder.append('.');
            }
            else if ("*".equals(match)) {
                pattern_builder.append(".*");
            }
            else if (match.startsWith("{") && match.endsWith("}")) {
                int colonIdx = match.indexOf(':');
                if (colonIdx == -1) {
                    pattern_builder.append(DEFAULT_VARIABLE_PATTERN);
                    this.variableNames.add(matcher.group(1));
                }
                else {
                    String variablePattern = match.substring(colonIdx + 1, match.length() - 1);
                    pattern_builder.append('(');
                    pattern_builder.append(variablePattern);
                    pattern_builder.append(')');
                    String variableName = match.substring(1, colonIdx);
                    this.variableNames.add(variableName);
                }
            }
            end = matcher.end();
        }
        // No glob pattern was found, this is an exact String match
        if (end == 0) {
            this.exactMatch = true;
            this.pattern = null;
        }
        else {
            this.exactMatch = false;
            pattern_builder.append(quote(pattern, end, pattern.length()));
            this.pattern = Pattern.compile(pattern_builder.toString(),
                                           Pattern.DOTALL | (this.caseSensitive ? 0 : Pattern.CASE_INSENSITIVE));
        }
    }

    fn quote(&self,s:String,start:usize,end:usize)->String {
        if start == end {
            return "".to_string();
        }
        return self.quote_the_str(s.substring(start, end).to_string());
    }
    fn  quote_the_str(&self,s:String)->String {

        let mut slash_e_index = s.find("\\E");
        if slash_e_index.is_none() {
            return "\\Q".to_string() + &*s + "\\E";
        }

        let mut sb = String::from("");
        sb.push_str("\\Q");
        let mut current = 0;
        let mut slash_e_index = 0;
        let mut current_str = s.clone();
        loop {
            let find_result = current_str.find("\\E");
            if find_result.is_none() {
                break;
            }
            slash_e_index = find_result.unwrap();
            sb.push_str(s.substring(current,slash_e_index));
            current = slash_e_index + 2;

            sb.push_str("\\E\\\\E\\Q");
            current_str = s.substring(current, s.len() - 1).parse().unwrap();
        }
        sb.push_str(s.substring(current, (s.len() as usize)-1));
        sb.push_str("\\E");
        return sb;
    }

    public boolean matchStrings(String str, @Nullable Map<String, String> uriTemplateVariables) {
        if (this.exactMatch) {
            return this.caseSensitive ? this.rawPattern.equals(str) : this.rawPattern.equalsIgnoreCase(str);
        }
        else if (this.pattern != null) {
            Matcher matcher = this.pattern.matcher(str);
            if (matcher.matches()) {
                if (uriTemplateVariables != null) {
                    if (this.variableNames.size() != matcher.groupCount()) {
                        throw new IllegalArgumentException("The number of capturing groups in the pattern segment " +
                        this.pattern + " does not match the number of URI template variables it defines, " +
                        "which can occur if capturing groups are used in a URI template regex. " +
                        "Use non-capturing groups instead.");
                    }
                    for (int i = 1; i <= matcher.groupCount(); i++) {
                        String name = this.variableNames.get(i - 1);
                        if (name.startsWith("*")) {
                            throw new IllegalArgumentException("Capturing patterns (" + name + ") are not " +
                            "supported by the AntPathMatcher. Use the PathPatternParser instead.");
                        }
                        String value = matcher.group(i);
                        uriTemplateVariables.put(name, value);
                    }
                }
                return true;
            }
        }
        return false;
    }

}
pub struct AntPathMatcher {
    pathSeparator:String,

     pathSeparatorPatternCache:PathSeparatorPatternCache,

    caseSensitive:bool,

    trimTokens: boolean ,


    cachePatterns: bool ,

      tokenizedPatternCache :HashMap<String, Vec<String>>,

      stringMatcherCache:HashMap<String, AntPathStringMatcher>
}

impl PathMatcher for AntPathMatcher{
    fn is_pattern(path: String) -> bool {
        todo!()
    }

    fn is_match(pattern: String, path: String) -> bool {
        todo!()
    }

    fn match_start(pattern: String, path: String) -> bool {
        todo!()
    }

    fn extract_path_within_pattern(pattern: String, path: String) -> String {
        todo!()
    }

    fn extract_uri_template_variables(pattern: String, path: String) -> HashMap<String, String> {
        todo!()
    }

    fn combine(pattern1: String, pattern2: String) -> String {
        todo!()
    }
}