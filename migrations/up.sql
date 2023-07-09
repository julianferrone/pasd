-- Add migration script here
CREATE TYPE status_type AS ENUM ('NotStarted', 'InProgress', 'Completed');
CREATE TABLE IF NOT EXISTS themes (
    theme_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    theme_status status_type NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);
CREATE TABLE IF NOT EXISTS objectives (
    objective_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    theme_id INT,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    CONSTRAINT fk_theme_id FOREIGN KEY (theme_id) REFERENCES themes(theme_id) ON DELETE NO ACTION
);
CREATE TABLE IF NOT EXISTS keyresults (
    keyresult_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    objective_id INT,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    CONSTRAINT fk_objective_id FOREIGN KEY (objective_id) REFERENCES objectives(objective_id) ON DELETE NO ACTION
);
CREATE TABLE IF NOT EXISTS initiatives (
    initiative_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    objective_id INT,
    initiative_status status_type NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    CONSTRAINT fk_objective_id FOREIGN KEY (objective_id) REFERENCES objectives(objective_id) ON DELETE NO ACTION
);
CREATE TABLE IF NOT EXISTS projects (
    project_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    objective_id INT,
    project_status status_type NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    CONSTRAINT fk_objective_id FOREIGN KEY (objective_id) REFERENCES objectives(objective_id) ON DELETE NO ACTION
);
CREATE TABLE IF NOT EXISTS tasks (
    task_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    project_id INT,
    task_status status_type NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    CONSTRAINT fk_project_id FOREIGN KEY (project_id) REFERENCES projects(project_id) ON DELETE NO ACTION
);
CREATE TABLE IF NOT EXISTS measurements (
    measurement_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    keyresult_id INT,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    CONSTRAINT fk_keyresult_id FOREIGN KEY (keyresult_id) REFERENCES keyresults(keyresult_id) ON DELETE NO ACTION
);