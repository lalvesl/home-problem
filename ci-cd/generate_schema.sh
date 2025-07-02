base_path="gutils/src/schema"
PRIMARY_KEY_PROC_MACRO='    #[sea_orm(primary_key, auto_increment = false)]'

function force_primarykey() {
  if [ ! -f "$1" ]; then
    echo "Error: File '$1' not found!"
    exit 1
  fi

  awk -v insert="$PRIMARY_KEY_PROC_MACRO" '
  BEGIN {
      # Flag to indicate if we are inside the Model struct
      in_model_struct = 0;
      # Flag to indicate if the insertion has been made for the current Model struct
      inserted = 0;
  }
  
  /pub struct Model \{/ {
      in_model_struct = 1;
      inserted = 0; # Reset for each new Model struct
      print;
      next;
  }
  
  in_model_struct == 1 && /pub [a-zA-Z_]+:/ && inserted == 0 {
      print insert;
      print;
      inserted = 1;
      next;
  }
  
  /^\}$/ { # End of struct or block
      if (in_model_struct == 1) {
          in_model_struct = 0;
      }
      print;
      next;
  }
  
  {
      print;
  }
  ' "$1" >"$1.tmp" && mv "$1.tmp" "$1"

}

# if ! cargo &>/dev/null; then
#     echo "Cargo could not be found, install by 'https://www.rust-lang.org/' this documentation"
#     exit 1
# fi

if ! sea-orm-cli --version &>/dev/null; then
  cargo install sea-orm-cli --profile dev
fi

if [ -f ".env" ]; then
  export $(cat .env | grep -v '^#' | grep "DB_\(CDP\)\|\(SALES_HUB\)\|\(SMART_KPI\)")
fi

if [ -z $DB_CDP_PG_DB ]; then
  echo "Enviroment variables not load"
  exit 1
fi

if ! rustfmt --version &>/dev/null; then
  rustsetup component add rustfmt
fi

for schema in $(echo "cdp_jobs cron_data_process cdp cdp_dyn_data_process portal-vendas quotes"); do
  echo "Generating entities from schema: '$schema'"
  schema_right_name="$(echo $schema | sed 's/\W/_/g')"
  locality="$base_path/$schema_right_name"

  rm -rf "$locality"
  mkdir "$locality"

  sea-orm-cli \
    generate entity \
    --database-url \
    $(
      # Consider only coordination_coordinator_customer_portfolio table, rest is desconsiderable
      if [ "portal-vendas" != $schema ]; then echo "postgresql://$DB_CDP_PG_USER:$(
        echo $DB_CDP_PG_PASS |
          base64 -d | jq -sRr @uri
      )@$DB_CDP_PG_HOST:$DB_CDP_PG_PORT/$DB_CDP_PG_DB"; fi
    ) \
    $(
      # Consider only coordination_coordinator_customer_portfolio table, rest is desconsiderable
      if [ "portal-vendas" = $schema ]; then echo "postgresql://$DB_SALES_HUB_PG_USER:$(
        echo $DB_SALES_HUB_PG_PASS |
          base64 -d | jq -sRr @uri
      )@$DB_SALES_HUB_PG_HOST:$DB_SALES_HUB_PG_PORT/$DB_SALES_HUB_PG_DB --tables coordinator_users"; fi
    ) \
    -s "$schema" \
    --model-extra-derives "Default" \
    --model-extra-attributes "allow(dead_code)" \
    --enum-extra-attributes "allow(dead_code)" \
    --with-serde both \
    -o "$locality"

  cp "$locality/mod.rs" "$locality/mod2.rs"
  cat "$locality/mod2.rs" | grep -v "prelude" >"$locality/mod.rs"
  rm -f "$locality/prelude.rs" "$locality/mod2.rs"

done

base_locally="$base_path/smart_kpi"
rm -rf $base_locally

for schema in $(echo "configuration hdb_catalog internal management monitoring options organization planning public references"); do
  echo "Generating entities from schema: '$schema'"
  schema_right_name="$(echo $schema | sed 's/\W/_/g')"
  locality="$base_path/smart_kpi/$schema_right_name"

  rm -rf "$locality"
  mkdir "$locality"

  sea-orm-cli \
    generate entity \
    --database-url \
    "postgresql://$DB_SMART_KPI_PG_USER:$(
      echo $DB_SMART_KPI_PG_PASS |
        base64 -d | jq -sRr @uri
    )@$DB_SMART_KPI_PG_HOST:$DB_SMART_KPI_PG_PORT/$DB_SMART_KPI_PG_DB" \
    -s "$schema" \
    --model-extra-derives "Default" \
    --model-extra-attributes "allow(dead_code)" \
    --enum-extra-attributes "allow(dead_code)" \
    --with-serde both \
    -o "$locality"

  cp "$locality/mod.rs" "$locality/mod2.rs"
  cat "$locality/mod2.rs" | grep -v "prelude" >"$locality/mod.rs"
  rm -f "$locality/prelude.rs" "$locality/mod2.rs"
  for i in $(find -type f | grep "$locality"); do
    echo $i
    force_primarykey $i
  done

  echo "pub mod $schema_right_name;" >>$base_locally/mod.rs
done

find
