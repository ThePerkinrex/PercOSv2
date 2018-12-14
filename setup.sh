#! /bin/bash
sed -i '.bak' 's%$PWD%'"$PWD"'%g' connect_template.lldb
mv connect_template.lldb connect.lldb
mv connect_template.lldb.bak connect_template.lldb