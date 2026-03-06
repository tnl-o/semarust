-- ============================================================================
-- Тестовые данные для CRUD Демо
-- ============================================================================
-- Этот скрипт добавляет тестовые данные в проект ID=1 (Demo Infrastructure)
-- ============================================================================

-- ============================================================================
-- Инвентари (3 штуки)
-- ============================================================================

INSERT INTO inventory (project_id, name, inventory_type, inventory_data, ssh_key_id, ssh_login, ssh_port, created) 
VALUES 
(1, 'Test Web Servers', 'static', 
'all:
  children:
    webservers:
      hosts:
        test-web1.example.com:
          ansible_user: ansible
          ansible_port: 22
        test-web2.example.com:
          ansible_user: ansible
          ansible_port: 22
        test-web3.example.com:
          ansible_user: ansible
          ansible_port: 22',
 1, 'ansible', 22, NOW()),

(1, 'Test Database Cluster', 'static',
'all:
  children:
    postgres:
      hosts:
        test-pg1.example.com:
          ansible_user: postgres
        test-pg2.example.com:
          ansible_user: postgres
    mysql:
      hosts:
        test-mysql1.example.com:
          ansible_user: mysql',
 1, 'postgres', 5432, NOW()),

(1, 'Test Staging Environment', 'static_json',
'{
  "all": {
    "children": {
      "staging": {
        "hosts": {
          "staging-app1": {
            "ansible_host": "192.168.1.100",
            "ansible_user": "ubuntu"
          },
          "staging-app2": {
            "ansible_host": "192.168.1.101",
            "ansible_user": "ubuntu"
          }
        }
      }
    }
  }
}',
 1, 'ubuntu', 22, NOW());

-- ============================================================================
-- Репозитории (2 штуки)
-- ============================================================================

INSERT INTO repository (project_id, name, git_url, git_type, git_branch, key_id, created)
VALUES
(1, 'Test Ansible Playbooks', 'https://github.com/ansible/ansible-examples.git', 'git', 'main', 1, NOW()),

(1, 'Test Infrastructure Code', 'https://github.com/hashicorp/terraform-guides.git', 'git', 'master', 1, NOW());

-- ============================================================================
-- Окружения (1 штука)
-- ============================================================================

INSERT INTO environment (project_id, name, json, created)
VALUES
(1, 'Test Environment Variables', '{
  "env": "test",
  "debug": true,
  "log_level": "debug",
  "max_connections": 100,
  "timeout": 30,
  "retry_count": 3,
  "backup_enabled": false,
  "monitoring_enabled": true
}', NOW());

-- ============================================================================
-- Ключи доступа (1 дополнительный)
-- ============================================================================

INSERT INTO access_key (project_id, name, type, login_password_login, login_password_password, created)
VALUES
(1, 'Test Login/Password Key', 'login_password', 'testuser', 'testpass123', NOW());

-- ============================================================================
-- Шаблоны (2 штуки)
-- ============================================================================

INSERT INTO template (project_id, inventory_id, repository_id, environment_id, name, description, playbook, arguments, allow_override_args_in_task, git_branch, diff, created)
VALUES
(1, 
 (SELECT id FROM inventory WHERE name = 'Test Web Servers' LIMIT 1),
 (SELECT id FROM repository WHERE name = 'Test Ansible Playbooks' LIMIT 1),
 (SELECT id FROM environment WHERE name = 'Test Environment Variables' LIMIT 1),
 'Test Web Server Deployment',
 'Шаблон для деплоя на тестовые веб-серверы',
 'deploy-webservers.yml',
 '["--verbose"]',
 TRUE,
 'main',
 TRUE,
 NOW()),

(1,
 (SELECT id FROM inventory WHERE name = 'Test Database Cluster' LIMIT 1),
 (SELECT id FROM repository WHERE name = 'Test Ansible Playbooks' LIMIT 1),
 NULL,
 'Test Database Backup',
 'Шаблон для резервного копирования тестовых БД',
 'backup-databases.yml',
 '[]',
 FALSE,
 'main',
 FALSE,
 NOW());

-- ============================================================================
-- Проверка созданных данных
-- ============================================================================

SELECT '✅ Инвентари:' AS info;
SELECT id, name, inventory_type FROM inventory WHERE project_id = 1 ORDER BY id;

SELECT '✅ Репозитории:' AS info;
SELECT id, name, git_url FROM repository WHERE project_id = 1 ORDER BY id;

SELECT '✅ Окружения:' AS info;
SELECT id, name FROM environment WHERE project_id = 1 ORDER BY id;

SELECT '✅ Ключи:' AS info;
SELECT id, name, type FROM access_key WHERE project_id = 1 ORDER BY id;

SELECT '✅ Шаблоны:' AS info;
SELECT id, name, playbook FROM template WHERE project_id = 1 ORDER BY id;

-- ============================================================================
-- Итоговая информация
-- ============================================================================

SELECT '
═══════════════════════════════════════════════════════
✅ ТЕСТОВЫЕ ДАННЫЕ УСПЕШНО СОЗДАНЫ!
═══════════════════════════════════════════════════════

Проект: Demo Infrastructure (ID=1)

Добавлено:
  📦 Инвентари: 3
    - Test Web Servers
    - Test Database Cluster
    - Test Staging Environment
  
  📚 Репозитории: 2
    - Test Ansible Playbooks
    - Test Infrastructure Code
  
  ⚙️ Окружения: 1
    - Test Environment Variables
  
  🔑 Ключи: 1 (дополнительный)
    - Test Login/Password Key
  
  📋 Шаблоны: 2
    - Test Web Server Deployment
    - Test Database Backup

Для проверки выполните:
  ./test-all-crud.sh

Для просмотра в UI:
  http://localhost/demo-crud.html
═══════════════════════════════════════════════════════
' AS result;
