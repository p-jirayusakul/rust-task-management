CREATE TABLE "master_data_task_status" (
  "id" bigint UNIQUE PRIMARY KEY NOT NULL,
  "title" varchar(255) NOT NULL,
  "code" varchar(50) UNIQUE NOT NULL,
  "active" boolean NOT NULL DEFAULT true,
  "created_by" bigint NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp,
  "updated_by" bigint
);

CREATE TABLE "master_data_role" (
  "id" bigint UNIQUE PRIMARY KEY NOT NULL,
  "title" varchar(255) NOT NULL,
  "code" varchar(50) UNIQUE NOT NULL,
  "active" boolean NOT NULL DEFAULT true,
  "created_by" bigint NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp,
  "updated_by" bigint
);

CREATE TABLE "master_data_priority_levels" (
  "id" bigint UNIQUE PRIMARY KEY NOT NULL,
  "seq" integer UNIQUE NOT NULL,
  "title" varchar(255) NOT NULL,
  "code" varchar(50) UNIQUE NOT NULL,
  "active" boolean NOT NULL DEFAULT true,
  "created_by" bigint NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp,
  "updated_by" bigint
);

CREATE TABLE "users" (
  "id" bigint UNIQUE PRIMARY KEY NOT NULL,
  "username" varchar(255) UNIQUE NOT NULL,
  "password" text NOT NULL,
  "role_id" bigint,
  "created_by" bigint NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp,
  "updated_by" bigint
);

CREATE TABLE "task" (
  "id" bigint UNIQUE PRIMARY KEY NOT NULL,
  "title" varchar(255) NOT NULL,
  "description" text,
  "task_status_id" bigint,
  "priority_levels_id" bigint,
  "created_by" bigint NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp,
  "updated_by" bigint
);

CREATE INDEX "master_data_task_status_id_idx" ON "master_data_task_status" USING BTREE ("id");

CREATE INDEX "master_data_task_status_code_idx" ON "master_data_task_status" USING BTREE ("code");

CREATE INDEX "master_data_role_id_idx" ON "master_data_role" USING BTREE ("id");

CREATE INDEX "master_data_role_code_idx" ON "master_data_role" USING BTREE ("code");

CREATE INDEX "master_priority_levels_id_idx" ON "master_data_priority_levels" USING BTREE ("id");

CREATE INDEX "master_priority_levels_code_idx" ON "master_data_priority_levels" USING BTREE ("code");

CREATE INDEX "users_id_idx" ON "users" USING BTREE ("id");

CREATE INDEX "users_username_idx" ON "users" USING BTREE ("username");

CREATE INDEX "task_id_idx" ON "task" USING BTREE ("id");

CREATE INDEX "products_title_idx" ON "task" USING BTREE ("title");

CREATE INDEX "task_task_status_id_idx" ON "task" USING BTREE ("task_status_id");

CREATE INDEX "task_priority_levels_id_idx" ON "task" USING BTREE ("priority_levels_id");

COMMENT ON COLUMN "master_data_task_status"."id" IS 'snowflake id';

COMMENT ON COLUMN "master_data_task_status"."title" IS 'ชื่อสถานะงาน เช่น รอเริ่มงาน';

COMMENT ON COLUMN "master_data_task_status"."code" IS 'รหัสย่อ เช่น PENDING';

COMMENT ON COLUMN "master_data_task_status"."active" IS 'เปิด/ปิด ใช้งาน';

COMMENT ON COLUMN "master_data_task_status"."created_by" IS 'คนที่สร้าง';

COMMENT ON COLUMN "master_data_task_status"."created_at" IS 'วันที่สร้าง';

COMMENT ON COLUMN "master_data_task_status"."updated_at" IS 'วันที่อัพเดท';

COMMENT ON COLUMN "master_data_task_status"."updated_by" IS 'คนที่อัพเดท';

COMMENT ON COLUMN "master_data_role"."id" IS 'snowflake id';

COMMENT ON COLUMN "master_data_role"."title" IS 'ชื่อตำแหน่ง เช่น Admin';

COMMENT ON COLUMN "master_data_role"."code" IS 'รหัสย่อ เช่น ADMIN';

COMMENT ON COLUMN "master_data_role"."active" IS 'เปิด/ปิด ใช้งาน';

COMMENT ON COLUMN "master_data_role"."created_by" IS 'คนที่สร้าง';

COMMENT ON COLUMN "master_data_role"."created_at" IS 'วันที่สร้าง';

COMMENT ON COLUMN "master_data_role"."updated_at" IS 'วันที่อัพเดท';

COMMENT ON COLUMN "master_data_role"."updated_by" IS 'คนที่อัพเดท';

COMMENT ON COLUMN "master_data_priority_levels"."id" IS 'snowflake id';

COMMENT ON COLUMN "master_data_priority_levels"."seq" IS 'ลำดับ';

COMMENT ON COLUMN "master_data_priority_levels"."title" IS 'ชื่อความสำคัญ เช่น Medium';

COMMENT ON COLUMN "master_data_priority_levels"."code" IS 'รหัสย่อ เช่น P3';

COMMENT ON COLUMN "master_data_priority_levels"."active" IS 'เปิด/ปิด ใช้งาน';

COMMENT ON COLUMN "master_data_priority_levels"."created_by" IS 'คนที่สร้าง';

COMMENT ON COLUMN "master_data_priority_levels"."created_at" IS 'วันที่สร้าง';

COMMENT ON COLUMN "master_data_priority_levels"."updated_at" IS 'วันที่อัพเดท';

COMMENT ON COLUMN "master_data_priority_levels"."updated_by" IS 'คนที่อัพเดท';

COMMENT ON COLUMN "users"."id" IS 'snowflake id';

COMMENT ON COLUMN "users"."username" IS 'username';

COMMENT ON COLUMN "users"."password" IS 'password';

COMMENT ON COLUMN "users"."role_id" IS 'foreign key table master_data_role';

COMMENT ON COLUMN "users"."created_by" IS 'คนที่สร้าง';

COMMENT ON COLUMN "users"."created_at" IS 'วันที่สร้าง';

COMMENT ON COLUMN "users"."updated_at" IS 'วันที่อัพเดท';

COMMENT ON COLUMN "users"."updated_by" IS 'คนที่อัพเดท';

COMMENT ON COLUMN "task"."id" IS 'snowflake id';

COMMENT ON COLUMN "task"."title" IS 'ชื่องาน เช่น ทดสอบ-1234';

COMMENT ON COLUMN "task"."description" IS 'รายละเอียดเพิ่มเติม';

COMMENT ON COLUMN "task"."task_status_id" IS 'foreign key table master_data_task_status';

COMMENT ON COLUMN "task"."priority_levels_id" IS 'foreign key table master_data_priority_levels';

COMMENT ON COLUMN "task"."created_by" IS 'คนที่สร้าง';

COMMENT ON COLUMN "task"."created_at" IS 'วันที่สร้าง';

COMMENT ON COLUMN "task"."updated_at" IS 'วันที่อัพเดท';

COMMENT ON COLUMN "task"."updated_by" IS 'คนที่อัพเดท';

ALTER TABLE "task" ADD FOREIGN KEY ("task_status_id") REFERENCES "master_data_task_status" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "task" ADD FOREIGN KEY ("priority_levels_id") REFERENCES "master_data_priority_levels" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "task" ADD FOREIGN KEY ("created_by") REFERENCES "users" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "task" ADD FOREIGN KEY ("updated_by") REFERENCES "users" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

ALTER TABLE "users" ADD FOREIGN KEY ("role_id") REFERENCES "master_data_role" ("id") ON DELETE NO ACTION ON UPDATE CASCADE;

-- init data
INSERT INTO master_data_priority_levels(
	id, seq, title, code, active, created_by, created_at)
	VALUES (7250065510946050048, 1, 'Critical', 'P1', TRUE, 0, NOW());
INSERT INTO master_data_priority_levels(
	id, seq, title, code, active, created_by, created_at)
	VALUES (7250065953734529024, 2, 'High', 'P2', TRUE, 0, NOW());
INSERT INTO master_data_priority_levels(
	id, seq, title, code, active, created_by, created_at)
	VALUES (7250065969870016512, 3, 'Medium', 'P3', TRUE, 0, NOW());
INSERT INTO master_data_priority_levels(
	id, seq, title, code, active, created_by, created_at)
	VALUES (7250065986953416704, 4, 'Low', 'P4', TRUE, 0, NOW());
INSERT INTO master_data_priority_levels(
	id, seq, title, code, active, created_by, created_at)
	VALUES (7250066005521600512, 5, 'Lowest', 'P5', TRUE, 0, NOW());

-- status
INSERT INTO master_data_task_status(
	id, title, code, active, created_by)
	VALUES (7250066646188953600, 'Pending', 'PENDING', TRUE, 0);

INSERT INTO master_data_task_status(
	id, title, code, active, created_by)
	VALUES (7250066663482068992, 'In Progress', 'IN_PROGRESS', TRUE, 0);

INSERT INTO master_data_task_status(
	id, title, code, active, created_by)
	VALUES (7250066683811860480, 'Completed', 'COMPLETED', TRUE, 0);

-- role
INSERT INTO master_data_role(
	id, title, code, active, created_by)
	VALUES (7250548959330963456, 'Admin', 'ADMIN', TRUE, 0);

INSERT INTO master_data_role(
	id, title, code, active, created_by)
	VALUES (7250549955788541952, 'Manager', 'MANAGER', TRUE, 0);

INSERT INTO master_data_role(
	id, title, code, active, created_by)
	VALUES (7250549977582145536, 'Member', 'MEMBER', TRUE, 0);

INSERT INTO master_data_role(
	id, title, code, active, created_by)
	VALUES (7250549998956318720, 'Viewer', 'VIEWER', TRUE, 0);

-- user
INSERT INTO users(
	id, username, password, role_id, created_by, created_at, updated_at, updated_by)
	VALUES (1844994649115070464, 'admin', '$2a$10$F25qV8QFjFQSdaGKmZ4sqOehkxmws12WyQV8wyOqLhQ1O8Pp7CM9G', 7250548959330963456, 0, NOW(), NOW(), NULL);

INSERT INTO users(
	id, username, password, role_id, created_by, created_at, updated_at, updated_by)
	VALUES (1844995500256792576, 'manager', '$2a$10$UHdFeVeOsl7g83xaZ2N4SOlLpvuGjGEb33DopZbCPfTScfFmfgR3W', 7250549955788541952, 0, NOW(), NOW(), NULL);

INSERT INTO users(
	id, username, password, role_id, created_by, created_at, updated_at, updated_by)
	VALUES (1844995683120058368, 'member1', '$2a$10$MCuKU9dtuRNhjJPWvfFXseLJKkTC1t6Us0HrNJijDvuTdmrXeu6sK', 7250549977582145536, 0, NOW(), NOW(), NULL);

INSERT INTO users(
	id, username, password, role_id, created_by, created_at, updated_at, updated_by)
	VALUES (1844995732965167104, 'member2', '$2a$10$cvA/j.VZq/0T4Ql.51UUxebNErZ6nuklGKfAtZRt6td5clv/fY1ci', 7250549977582145536, 0, NOW(), NOW(), NULL);