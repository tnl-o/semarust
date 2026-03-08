/**
 * Semaphore UI - CRUD Демо
 * Расширенная версия с поддержкой всех сущностей
 * Инвентарь, Репозитории, Окружения, Ключи, Шаблоны, Задачи, Расписания
 */

// ============================================================================
// Формы для создания/редактирования сущностей
// ============================================================================

function buildInventoryForm(inventory = null) {
    return `
        <form id="inventory-form">
            <div class="form-group">
                <label for="inventory-name">Название *</label>
                <input type="text" id="inventory-name" name="name" required
                       value="${escapeHtml(inventory?.name || '')}">
            </div>
            <div class="form-group">
                <label for="inventory-type">Тип *</label>
                <select id="inventory-type" name="inventory_type" required>
                    <option value="static" ${inventory?.inventory_type === 'static' ? 'selected' : ''}>Static (YAML/INI)</option>
                    <option value="static_json" ${inventory?.inventory_type === 'static_json' ? 'selected' : ''}>Static JSON</option>
                    <option value="file" ${inventory?.inventory_type === 'file' ? 'selected' : ''}>File</option>
                </select>
            </div>
            <div class="form-group">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                    <label for="inventory-data">Данные инвентаря *</label>
                    <button type="button" class="btn btn-secondary btn-sm" onclick="showPlaybookSelector()" style="font-size: 12px; padding: 4px 8px;">
                        📋 Выбрать из репозитория
                    </button>
                </div>
                <textarea id="inventory-data" name="inventory_data" required
                          style="min-height: 200px; font-family: monospace;">${escapeHtml(inventory?.inventory_data || '')}</textarea>
                <small>Формат: YAML, INI или JSON. Используйте кнопку выше для выбора из репозитория</small>
            </div>
            <div class="form-group">
                <label for="inventory-ssh-login">SSH Login</label>
                <input type="text" id="inventory-ssh-login" name="ssh_login"
                       value="${escapeHtml(inventory?.ssh_login || 'ansible')}">
            </div>
            <div class="form-group">
                <label for="inventory-ssh-port">SSH Port</label>
                <input type="number" id="inventory-ssh-port" name="ssh_port"
                       value="${inventory?.ssh_port || 22}">
            </div>
            <div class="form-actions">
                <button type="button" class="btn btn-secondary" onclick="closeModal()">Отмена</button>
                <button type="submit" class="btn btn-primary">Сохранить</button>
            </div>
        </form>
    `;
}

function buildRepositoryForm(repo = null) {
    return `
        <form id="repository-form">
            <div class="form-group">
                <label for="repo-name">Название *</label>
                <input type="text" id="repo-name" name="name" required 
                       value="${escapeHtml(repo?.name || '')}">
            </div>
            <div class="form-group">
                <label for="repo-url">Git URL *</label>
                <input type="url" id="repo-url" name="git_url" required 
                       value="${escapeHtml(repo?.git_url || '')}"
                       placeholder="https://github.com/user/repo.git">
            </div>
            <div class="form-group">
                <label for="repo-type">Тип</label>
                <select id="repo-type" name="git_type">
                    <option value="git" ${repo?.git_type === 'git' ? 'selected' : ''}>Git (SSH)</option>
                    <option value="https" ${repo?.git_type === 'https' ? 'selected' : ''}>HTTPS</option>
                    <option value="http" ${repo?.git_type === 'http' ? 'selected' : ''}>HTTP</option>
                    <option value="file" ${repo?.git_type === 'file' ? 'selected' : ''}>File</option>
                </select>
            </div>
            <div class="form-group">
                <label for="repo-branch">Ветвь</label>
                <input type="text" id="repo-branch" name="git_branch" 
                       value="${escapeHtml(repo?.git_branch || 'main')}"
                       placeholder="main">
            </div>
            <div class="form-actions">
                <button type="button" class="btn btn-secondary" onclick="closeModal()">Отмена</button>
                <button type="submit" class="btn btn-primary">Сохранить</button>
            </div>
        </form>
    `;
}

function buildEnvironmentForm(env = null) {
    // Парсим JSON для красивого отображения
    let jsonStr = env?.json || '{}';
    try {
        if (typeof jsonStr === 'string') {
            jsonStr = JSON.stringify(JSON.parse(jsonStr), null, 2);
        }
    } catch (e) {}
    
    return `
        <form id="environment-form">
            <div class="form-group">
                <label for="env-name">Название *</label>
                <input type="text" id="env-name" name="name" required 
                       value="${escapeHtml(env?.name || '')}">
            </div>
            <div class="form-group">
                <label for="env-json">Переменные (JSON) *</label>
                <textarea id="env-json" name="json" required 
                          style="min-height: 300px; font-family: monospace;">${escapeHtml(jsonStr)}</textarea>
                <small>Формат: JSON с переменными окружения</small>
            </div>
            <div class="form-actions">
                <button type="button" class="btn btn-secondary" onclick="closeModal()">Отмена</button>
                <button type="submit" class="btn btn-primary">Сохранить</button>
            </div>
        </form>
    `;
}

function buildKeyForm(key = null) {
    const isSSH = !key || key.type === 'ssh';
    const isLoginPassword = key?.type === 'login_password';
    
    return `
        <form id="key-form">
            <div class="form-group">
                <label for="key-name">Название *</label>
                <input type="text" id="key-name" name="name" required 
                       value="${escapeHtml(key?.name || '')}">
            </div>
            <div class="form-group">
                <label for="key-type">Тип *</label>
                <select id="key-type" name="type" required onchange="toggleKeyFields()">
                    <option value="ssh" ${key?.type === 'ssh' ? 'selected' : ''}>SSH Key</option>
                    <option value="login_password" ${key?.type === 'login_password' ? 'selected' : ''}>Login/Password</option>
                    <option value="none" ${key?.type === 'none' ? 'selected' : ''}>None (без аутентификации)</option>
                </select>
            </div>
            
            <div id="ssh-key-fields" style="${isSSH ? '' : 'display: none;'}">
                <div class="form-group">
                    <label for="key-ssh">SSH Private Key *</label>
                    <textarea id="key-ssh" name="ssh_key" 
                              style="min-height: 200px; font-family: monospace;">${escapeHtml(key?.ssh_key || '')}</textarea>
                    <small>Вставьте содержимое файла id_rsa</small>
                </div>
                <div class="form-group">
                    <label for="key-passphrase">Passphrase (опционально)</label>
                    <input type="password" id="key-passphrase" name="ssh_passphrase" 
                           value="${escapeHtml(key?.ssh_passphrase || '')}">
                </div>
            </div>
            
            <div id="login-password-fields" style="${isLoginPassword ? '' : 'display: none;'}">
                <div class="form-group">
                    <label for="key-login">Login *</label>
                    <input type="text" id="key-login" name="login_password_login" 
                           value="${escapeHtml(key?.login_password_login || '')}">
                </div>
                <div class="form-group">
                    <label for="key-password">Password *</label>
                    <input type="password" id="key-password" name="login_password_password" 
                           value="${escapeHtml(key?.login_password_password || '')}">
                </div>
            </div>
            
            <div class="form-actions">
                <button type="button" class="btn btn-secondary" onclick="closeModal()">Отмена</button>
                <button type="submit" class="btn btn-primary">Сохранить</button>
            </div>
        </form>
    `;
}

function buildTemplateForm(template = null, inventories = [], repositories = [], environments = []) {
    return `
        <form id="template-form">
            <div class="form-group">
                <label for="template-name">Название *</label>
                <input type="text" id="template-name" name="name" required 
                       value="${escapeHtml(template?.name || '')}">
            </div>
            <div class="form-group">
                <label for="template-playbook">Playbook *</label>
                <input type="text" id="template-playbook" name="playbook" required 
                       value="${escapeHtml(template?.playbook || '')}"
                       placeholder="site.yml">
            </div>
            <div class="form-group">
                <label for="template-description">Описание</label>
                <textarea id="template-description" name="description">${escapeHtml(template?.description || '')}</textarea>
            </div>
            
            <div class="form-group">
                <label for="template-inventory">Инвентарь</label>
                <select id="template-inventory" name="inventory_id">
                    <option value="">-- Не выбрано --</option>
                    ${inventories.map(inv => `<option value="${inv.id}" ${template?.inventory_id === inv.id ? 'selected' : ''}>${escapeHtml(inv.name)}</option>`).join('')}
                </select>
            </div>
            
            <div class="form-group">
                <label for="template-repository">Репозиторий</label>
                <select id="template-repository" name="repository_id">
                    <option value="">-- Не выбрано --</option>
                    ${repositories.map(repo => `<option value="${repo.id}" ${template?.repository_id === repo.id ? 'selected' : ''}>${escapeHtml(repo.name)}</option>`).join('')}
                </select>
            </div>
            
            <div class="form-group">
                <label for="template-environment">Окружение</label>
                <select id="template-environment" name="environment_id">
                    <option value="">-- Не выбрано --</option>
                    ${environments.map(env => `<option value="${env.id}" ${template?.environment_id === env.id ? 'selected' : ''}>${escapeHtml(env.name)}</option>`).join('')}
                </select>
            </div>
            
            <div class="form-group">
                <label>
                    <input type="checkbox" name="allow_override_args_in_task" ${template?.allow_override_args_in_task ? 'checked' : ''}>
                    Разрешить передачу аргументов в задаче
                </label>
            </div>
            
            <div class="form-actions">
                <button type="button" class="btn btn-secondary" onclick="closeModal()">Отмена</button>
                <button type="submit" class="btn btn-primary">Сохранить</button>
            </div>
        </form>
    `;
}

function buildScheduleForm(schedule = null, templates = []) {
    return `
        <form id="schedule-form">
            <div class="form-group">
                <label for="schedule-name">Название *</label>
                <input type="text" id="schedule-name" name="name" required 
                       value="${escapeHtml(schedule?.name || '')}">
            </div>
            <div class="form-group">
                <label for="schedule-template">Шаблон *</label>
                <select id="schedule-template" name="template_id" required>
                    <option value="">-- Выберите шаблон --</option>
                    ${templates.map(tpl => `<option value="${tpl.id}" ${schedule?.template_id === tpl.id ? 'selected' : ''}>${escapeHtml(tpl.name)}</option>`).join('')}
                </select>
            </div>
            <div class="form-group">
                <label for="schedule-cron">Cron выражение *</label>
                <input type="text" id="schedule-cron" name="cron" required 
                       value="${escapeHtml(schedule?.cron || '')}"
                       placeholder="0 2 * * *">
                <small>Пример: 0 2 * * * (ежедневно в 2:00)</small>
            </div>
            <div class="form-group">
                <label>
                    <input type="checkbox" name="active" ${schedule?.active !== false ? 'checked' : ''}>
                    Активно
                </label>
            </div>
            <div class="form-actions">
                <button type="button" class="btn btn-secondary" onclick="closeModal()">Отмена</button>
                <button type="submit" class="btn btn-primary">Сохранить</button>
            </div>
        </form>
    `;
}

// ============================================================================
// Переключение полей для ключей
// ============================================================================

function toggleKeyFields() {
    const keyType = document.getElementById('key-type')?.value;
    const sshFields = document.getElementById('ssh-key-fields');
    const loginPasswordFields = document.getElementById('login-password-fields');
    
    if (sshFields) {
        sshFields.style.display = keyType === 'ssh' ? '' : 'none';
    }
    if (loginPasswordFields) {
        loginPasswordFields.style.display = keyType === 'login_password' ? '' : 'none';
    }
}

// ============================================================================
// Модальные окна для создания сущностей
// ============================================================================

function showCreateInventoryModal() {
    openModal('Создать инвентарь', buildInventoryForm());
    
    document.getElementById('inventory-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        const formData = new FormData(e.target);
        const inventoryData = {
            name: formData.get('name'),
            inventory_type: formData.get('inventory_type'),
            inventory_data: formData.get('inventory_data'),
            ssh_login: formData.get('ssh_login'),
            ssh_port: parseInt(formData.get('ssh_port')),
        };
        
        try {
            await createInventory(inventoryData);
            closeModal();
        } catch (error) {
            // Error already shown via toast
        }
    });
}

function showCreateRepositoryModal() {
    openModal('Создать репозиторий', buildRepositoryForm());
    
    document.getElementById('repository-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        const formData = new FormData(e.target);
        const repoData = {
            name: formData.get('name'),
            git_url: formData.get('git_url'),
            git_type: formData.get('git_type'),
            git_branch: formData.get('git_branch'),
        };
        
        try {
            await createRepository(repoData);
            closeModal();
        } catch (error) {
            // Error already shown via toast
        }
    });
}

function showCreateEnvironmentModal() {
    openModal('Создать окружение', buildEnvironmentForm());
    
    document.getElementById('environment-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        const formData = new FormData(e.target);
        const envData = {
            name: formData.get('name'),
            json: formData.get('json'),
        };
        
        try {
            await createEnvironment(envData);
            closeModal();
        } catch (error) {
            // Error already shown via toast
        }
    });
}

function showCreateKeyModal() {
    openModal('Создать ключ доступа', buildKeyForm());
    toggleKeyFields();
    
    document.getElementById('key-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        const formData = new FormData(e.target);
        const keyType = formData.get('type');
        
        const keyData = {
            name: formData.get('name'),
            type: keyType,
        };
        
        if (keyType === 'ssh') {
            keyData.ssh_key = formData.get('ssh_key');
            keyData.ssh_passphrase = formData.get('ssh_passphrase');
        } else if (keyType === 'login_password') {
            keyData.login_password_login = formData.get('login_password_login');
            keyData.login_password_password = formData.get('login_password_password');
        }
        
        try {
            await createKey(keyData);
            closeModal();
        } catch (error) {
            // Error already shown via toast
        }
    });
}

function showCreateTemplateModal() {
    // Загружаем зависимости
    Promise.all([
        apiRequest(`/project/${CURRENT_PROJECT_ID}/inventory`).catch(() => []),
        apiRequest(`/project/${CURRENT_PROJECT_ID}/repository`).catch(() => []),
        apiRequest(`/project/${CURRENT_PROJECT_ID}/environment`).catch(() => [])
    ]).then(([inventories, repositories, environments]) => {
        openModal('Создать шаблон', buildTemplateForm(null, inventories, repositories, environments));
        
        document.getElementById('template-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            const formData = new FormData(e.target);
            const templateData = {
                name: formData.get('name'),
                playbook: formData.get('playbook'),
                description: formData.get('description'),
                inventory_id: formData.get('inventory_id') ? parseInt(formData.get('inventory_id')) : null,
                repository_id: formData.get('repository_id') ? parseInt(formData.get('repository_id')) : null,
                environment_id: formData.get('environment_id') ? parseInt(formData.get('environment_id')) : null,
                allow_override_args_in_task: formData.get('allow_override_args_in_task') === 'on',
            };
            
            try {
                await createTemplate(templateData);
                closeModal();
            } catch (error) {
                // Error already shown via toast
            }
        });
    });
}

function showCreateScheduleModal() {
    apiRequest(`/project/${CURRENT_PROJECT_ID}/templates`).then(templates => {
        openModal('Создать расписание', buildScheduleForm(null, templates));
        
        document.getElementById('schedule-form').addEventListener('submit', async (e) => {
            e.preventDefault();
            const formData = new FormData(e.target);
            const scheduleData = {
                name: formData.get('name'),
                template_id: parseInt(formData.get('template_id')),
                cron: formData.get('cron'),
                active: formData.get('active') === 'on',
            };
            
            try {
                await createSchedule(scheduleData);
                closeModal();
            } catch (error) {
                // Error already shown via toast
            }
        });
    });
}

// ============================================================================
// Экспорт функций
// ============================================================================

window.toggleKeyFields = toggleKeyFields;
window.showCreateInventoryModal = showCreateInventoryModal;
window.showCreateRepositoryModal = showCreateRepositoryModal;
window.showCreateEnvironmentModal = showCreateEnvironmentModal;
window.showCreateKeyModal = showCreateKeyModal;
window.showCreateTemplateModal = showCreateTemplateModal;
window.showCreateScheduleModal = showCreateScheduleModal;

// ============================================================================
// Playbook Selector
// ============================================================================

async function showPlaybookSelector() {
    try {
        // Загружаем список playbook из репозиториев
        const playbooks = await apiRequest(`/projects/${CURRENT_PROJECT_ID}/inventories/playbooks`);

        if (playbooks.length === 0) {
            showToast('Нет доступных playbook в репозиториях', 'warning');
            return;
        }

        // Создаём модальное окно для выбора playbook
        openModal('Выберите playbook', `
            <div class="playbook-selector">
                <p>Выберите playbook из доступных в репозиториях:</p>
                <div class="playbook-list" style="max-height: 400px; overflow-y: auto;">
                    ${playbooks.map(pb => `
                        <div class="playbook-item" style="padding: 10px; border: 1px solid #ddd; margin-bottom: 8px; border-radius: 4px; cursor: pointer;"
                             onclick="selectPlaybook('${escapeHtml(pb)}')">
                            📄 ${escapeHtml(pb)}
                        </div>
                    `).join('')}
                </div>
                <div class="form-actions" style="margin-top: 15px;">
                    <button type="button" class="btn btn-secondary" onclick="closeModal()">Отмена</button>
                </div>
            </div>
        `);
    } catch (error) {
        showToast('Ошибка загрузки playbook: ' + error.message, 'error');
    }
}

function selectPlaybook(playbookPath) {
    // Вставляем путь к playbook в поле inventory_data
    const inventoryDataField = document.getElementById('inventory-data');
    if (inventoryDataField) {
        inventoryDataField.value = playbookPath;
    }
    closeModal();
    showToast(`Playbook ${playbookPath} выбран`, 'success');
}

window.showPlaybookSelector = showPlaybookSelector;
window.selectPlaybook = selectPlaybook;
