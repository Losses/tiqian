package org.tiqian.font;

class FontRoleFns {
    public static function usesLatinFace(role:FontRole):Bool return role == LatinText;
    public static function fontRoleNameUsesLatinFace(roleName:Null<String>):Bool return roleName == "LatinText";
}
